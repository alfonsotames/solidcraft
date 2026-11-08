//! A fold across a curved outline keeps the outline's own curves at the ends of its bend (#6748):
//! the model cache's sheet made from surfaces saves, for each bend whose zone ends follow the
//! outline, `end_curves`, each end's curves in its first panel's parameters. Older designs need no
//! transformation: a bend without them has straight ends, as before. The last release refuses a
//! design saved with this (the newer format number).
