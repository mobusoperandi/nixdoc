# Context

Popular programming languages tend to have documentation generators.
A documentation generator is a tool that typically reads source code and generates documentation in HTML or some other form.

The presence of a documentation generator for a programming language typically results in widespread use of it for the generation of documentation of libraries. 
The widespread use leads to prevalence of documentation of libraries. 
It also leads to familiarity with a certain documentation format, which makes documentation easier to read.

Nix does not have a documentation generator.
Nix suffers from a lack of documentation in its "standard library" and in the ecosystem. 
Existing documentation tends to vary in format and quality.

# Goals

Develop a documentation generator for the nix ecosystem.

# Example input

```nix
{
  ## Return if an attribute from nested attribute set exists.
  ## Example:
  ##
  ## ```
  ## let
  ##   x = {
  ##     a = {
  ##       b = 3;
  ##     };
  ##   };
  ## in assert hasAttrByPath ["a" "b"] x; true
  ## ```
  hasAttrByPath = attrPath: e:
    let attr = head attrPath;
    in
      if attrPath == [] then true
      else if e ? ${attr}
      then hasAttrByPath (tail attrPath) e.${attr}
      else false;
}
```

# Example output
