'''
File: generate_fst_basis.py
Author: Amber Converse
Purpose: Generates an input file for generate_fst based on a spaCy model 
'''

import argparse
import spacy

def main(vocab, spacy_model):
    '''
    STUB
    '''
    pass

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("input",
                    help="Input file path")
    parser.add_argument("spacy_model",
                    help="spaCy model name")
    args = parser.parse_args()

    main(args.input, args.spacy_model)