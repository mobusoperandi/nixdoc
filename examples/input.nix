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
    if attrPath == [ ] then true
    else if e ? ${attr}
    then hasAttrByPath (tail attrPath) e.${attr}
    else false;
  ## The system
  system = "x86_64-linux";
  ## The amount of cores
  buildCores = 5;
  ## Utilities
  utils = {
    ## This comment will not show up in documentation, at all.
    parse = builtins.fromJSON;
    ## Returns true if a provided number is greater than 3.
    isGreaterThan3 = n: n > 3;
  };
}
