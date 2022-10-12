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
