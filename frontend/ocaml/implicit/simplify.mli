type simplified =
  | SNothing
  | SEverything
  | SShape of Stages.simplified

val simplify: Stages.user -> simplified
