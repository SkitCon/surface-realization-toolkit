# surface-realization-toolkit
The goal of this project is to create a framework for creating a surface realization system from a vocabulary and an existing morphological analysis module.

**WORK IN PROGRESS:** Toolkit is currently unfinished

An FST is generated from a list of lemmas with all surface forms and their morphological categories, e.g.

```
eat: eat+V+SING+PRES, eats+V+PLU+PRES, ate+V+PAST, ...
cat: cat+N+SING, cats+N+PLU
...
```

This toolkit has been designed for Spanish, but with minimal adjustments, it should be applicable to any language. Input file to the FST generation code can be generated however desired, but I provide a template in Python which uses spaCy morphological analysis to generate the input file. All that is required is implementing a morphology extraction function.
