# Amalgate
Let's talk about programming languages for a second. Have you ever switched languages, and wished
that the language you were using had a feature from another?

Well, that's certainly how I feel whenever I pick up **bash**. Bash is, in my
humble opinion, the worst of many worlds of programming. Strange syntax rules,
a noticeable lack of features, inconsistency...

That's where **amalgate** comes into play.

## What does Amalgate do?
Amalgate adds new features to bash. To retain compatibility, all of the features that 
amalgate itself adds can generate a perfectly compatible bash source file.
However, certain concepts (like classes) simply do not translate 
well to Bash, so it provides an interface for modules to interact with the 
amalgate source.

## Amalgate Features

**Import**: Import will include a source file (usually another bash file) 
inline at the calling point. read() relies too much on filesystem structure.
Import will properly include the file during transpilation.

# Syntax
Amalgate commands go on their own line. The amalgate command
(=) lets the reader know that what follows is a command. So, 
you'd call import like 

```python
myFunction(){}
=import mathFunctions.sh
```

Amalgate has a command, confusingly enough, called '='. This lets you
access amalgate system commands and modules. So, for a module called 'profile',
you'd call it (with arguments) like 

```bash
myFunction(){}
== profile myFunction
```

You can call amalgate commands inline with `={`.

```
myFunction(){$1 + ={== date todau "MM/DD/YYYY"}
```

You can use non-module functions (like import) inline, but it's rare to do something like that.
# Example

**mathFunctions.bash**:

```BASH

function add() { echo $(($1 + $2)) }

```

**main.bash**
```BASH
=
```