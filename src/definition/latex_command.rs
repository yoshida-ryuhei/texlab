use texlab_workspace::*;
use futures_boxed::boxed;
use texlab_protocol::{LocationLink, TextDocumentPositionParams};
use texlab_syntax::*;

pub struct LatexCommandDefinitionProvider;

impl FeatureProvider for LatexCommandDefinitionProvider {
    type Params = TextDocumentPositionParams;
    type Output = Vec<LocationLink>;

    #[boxed]
    async fn execute<'a>(&'a self, request: &'a FeatureRequest<Self::Params>) -> Self::Output {
        let mut definitions = Vec::new();
        if let SyntaxTree::Latex(tree) = &request.document().tree {
            if let Some(LatexNode::Command(command)) = tree.find(request.params.position).last() {
                for document in request.related_documents() {
                    if let SyntaxTree::Latex(tree) = &document.tree {
                        tree.command_definitions
                            .iter()
                            .filter(|def| def.definition.name.text() == command.name.text())
                            .map(|def| LocationLink {
                                origin_selection_range: Some(command.range()),
                                target_uri: document.uri.clone().into(),
                                target_range: def.range(),
                                target_selection_range: def.range(),
                            })
                            .for_each(|def| definitions.push(def));

                        tree.math
                            .operators
                            .iter()
                            .filter(|op| op.definition.name.text() == command.name.text())
                            .map(|op| LocationLink {
                                origin_selection_range: Some(command.range()),
                                target_uri: document.uri.clone().into(),
                                target_range: op.range(),
                                target_selection_range: op.range(),
                            })
                            .for_each(|def| definitions.push(def));
                    }
                }
            }
        }
        definitions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use texlab_protocol::RangeExt;
    use texlab_protocol::{Position, Range};

    #[test]
    fn test_command_definition() {
        let links = test_feature(
            LatexCommandDefinitionProvider,
            FeatureSpec {
                files: vec![
                    FeatureSpec::file("foo.tex", "\\include{bar}\n\\foo"),
                    FeatureSpec::file("bar.tex", "\\newcommand{\\foo}{bar}"),
                    FeatureSpec::file("baz.tex", "\\newcommand{\\foo}{baz}"),
                ],
                main_file: "foo.tex",
                position: Position::new(1, 3),
                ..FeatureSpec::default()
            },
        );
        assert_eq!(
            links,
            vec![LocationLink {
                origin_selection_range: Some(Range::new_simple(1, 0, 1, 4)),
                target_uri: FeatureSpec::uri("bar.tex"),
                target_range: Range::new_simple(0, 0, 0, 22),
                target_selection_range: Range::new_simple(0, 0, 0, 22),
            }]
        );
    }

    #[test]
    fn test_math_operator() {
        let links = test_feature(
            LatexCommandDefinitionProvider,
            FeatureSpec {
                files: vec![
                    FeatureSpec::file("foo.tex", "\\include{bar}\n\\foo"),
                    FeatureSpec::file("bar.tex", "\\DeclareMathOperator{\\foo}{bar}"),
                    FeatureSpec::file("baz.tex", "\\DeclareMathOperator{\\foo}{baz}"),
                ],
                main_file: "foo.tex",
                position: Position::new(1, 3),
                ..FeatureSpec::default()
            },
        );
        assert_eq!(
            links,
            vec![LocationLink {
                origin_selection_range: Some(Range::new_simple(1, 0, 1, 4)),
                target_uri: FeatureSpec::uri("bar.tex"),
                target_range: Range::new_simple(0, 0, 0, 31),
                target_selection_range: Range::new_simple(0, 0, 0, 31)
            }]
        );
    }
}
