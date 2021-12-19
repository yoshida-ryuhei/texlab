#pragma once

#include <stdlib.h>
#include <string.h>

#define ALPHABET_SIZE 128

typedef struct trie {
    struct trie *children[ALPHABET_SIZE];
    int value;
} trie;

static trie *trie_new() {
    trie *root = (trie *)malloc(sizeof(trie));
    root->value = 0;
    memset(root->children, 0, sizeof(trie *) * ALPHABET_SIZE);
    return root;
}

static void trie_insert(trie *root, char *key, int value) {
    trie *node = root;
    for (int i = 0; key[i]; i++) {
        int c = (int)key[i];
        if (!node->children[c]) {
            node->children[c] = trie_new();
        }

        node = node->children[c];
    }

    node->value = value;
}

static void trie_insert_group(trie *root, char **keys, int value) {
    for (int i = 0; keys[i]; i++) {
        trie_insert(root, keys[i], value);
    }
}

static trie *trie_lookup(trie *root, int32_t c) {
    return root && (c < ALPHABET_SIZE) ? root->children[c] : NULL;
}

static void trie_free(trie *root) {
    for (int i = 0; i < ALPHABET_SIZE; i++) {
        if (root->children[i]) {
            trie_free(root->children[i]);
        }
    }

    free(root);
}