/* Other headers */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>


/* Headers */
#include "include/utils/dict.h"


typedef struct _Node Node;

typedef struct _Node {
    void *value;
    const char *key;
    Node *next;
} Node;

typedef struct _Dict {
    size_t size;
    Node **_storage;
} Dict;




Dict *dict_init() {
    /* Allocate storage */
    Node **stor = (Node **)calloc(31, sizeof(Node *));
    if(!stor) return NULL;

    /* Allocate struct */
    Dict *dictionary = calloc(1, sizeof(Dict));
    if(!dictionary){
        free(stor);
        return NULL;
    }

    dictionary->_storage = stor;
    dictionary->size = 31;

    return dictionary;
}


static size_t _hash_func(const char *key) {
    /* Get key length */
    size_t keylen = strlen(key);

    size_t hash = 11;
    for(int i = 0; i < keylen; ++i) {
        hash = hash * 31 + key[i];
    }

    return hash;
}



bool dict_insert(Dict *dictionary, const char *key, void *value) {
    /* Allocate Node */
    Node *new_node = calloc(1, sizeof(Node));
    if(!new_node) return NULL;

    /* Assign values */
    new_node->key = key;
    new_node->value = value;

    /* Calculate hash */
    size_t hashed_index = (_hash_func(key)) % 31;

    /* Check if slot in table is free */
    if(dictionary->_storage[hashed_index]) {
        /* Append Node to LL */
        Node *cur_node = dictionary->_storage[hashed_index];
        while(cur_node->next) cur_node = cur_node->next;
        cur_node->next = new_node;
        return false;
    } else {
        dictionary->_storage[hashed_index] = new_node;
        return true;
    }
}

void dict_action(Dict *dictionary, void (*action) (void *)) {
    /* Store length of list */
    const size_t dict_len = 31;

    for(int i = 0; i < dict_len ; ++i) {
        if(!dictionary->_storage[i])
            continue;

        /* Walk through LL and free all Nodes */
        Node *cur_node = dictionary->_storage[i];
        while(cur_node) {
            action(cur_node->value);
            cur_node = cur_node->next;
        }
    }
}


void *dict_get(Dict *dictionary,const char *key) {
    /* Calculate hash */
    size_t hashed_index = (_hash_func(key)) % 31;

    if(!dictionary->_storage[hashed_index]) {
        return NULL;
    } else {
        Node *cur_node = dictionary->_storage[hashed_index];
        while(cur_node) {
            /* Linear search for key */
            if(strcmp(cur_node->key, key) == 0)
                return cur_node->value;
            else
                cur_node = cur_node->next;
        }
        /* Could not find key */
       return NULL;
    }

}

void dict_free(Dict *dictionary) {
    /* Store length of list */
    const size_t dict_len = 31;

    for(int i = 0; i < dict_len ; ++i) {
        if(!dictionary->_storage[i])
            continue;

        /* Walk through LL and free all Nodes */
        Node *cur_node = dictionary->_storage[i];
        Node *prev_node = NULL;
        while(cur_node) {
            prev_node = cur_node;
            cur_node = cur_node->next;
            free(prev_node);
        }
    }
    /* Free table */
    free(dictionary->_storage);

    /* Free struct */
    free(dictionary);
}




