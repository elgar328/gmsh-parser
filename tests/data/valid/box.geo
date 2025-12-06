SetFactory("OpenCASCADE");

// --- 1. Create Box ---
Box(1) = {0, 0, 0, 1, 1, 1};

// --- 2. Identify Surfaces & Define Physical Groups ---
eps = 1e-3;

// X-axis faces (Left/Right)
s_left[]  = Surface In BoundingBox{-eps,  -eps,  -eps,   eps,   1+eps, 1+eps};
s_right[] = Surface In BoundingBox{1-eps, -eps,  -eps,   1+eps, 1+eps, 1+eps};

// Y-axis faces (Front/Back)
s_front[] = Surface In BoundingBox{-eps,  -eps,  -eps,   1+eps, eps,   1+eps};
s_back[]  = Surface In BoundingBox{-eps,  1-eps, -eps,   1+eps, 1+eps, 1+eps};

// Z-axis faces (Bottom/Top)
s_bottom[] = Surface In BoundingBox{-eps, -eps,  -eps,   1+eps, 1+eps, eps};
s_top[]    = Surface In BoundingBox{-eps, -eps,  1-eps,  1+eps, 1+eps, 1+eps};

// Assign names to surfaces for identification in the MSH file
Physical Surface("Left")   = {s_left[0]};
Physical Surface("Right")  = {s_right[0]};
Physical Surface("Front")  = {s_front[0]};
Physical Surface("Back")   = {s_back[0]};
Physical Surface("Bottom") = {s_bottom[0]};
Physical Surface("Top")    = {s_top[0]};
Physical Volume("TheBox") = {1};

// --- 3. Define Periodic Conditions ---
// Syntax: Periodic Surface {Slave} = {Master} Translate {dx, dy, dz};
Periodic Surface {s_right[0]} = {s_left[0]} Translate {1, 0, 0};
Periodic Surface {s_back[0]}  = {s_front[0]} Translate {0, 1, 0};
Periodic Surface {s_top[0]}   = {s_bottom[0]} Translate {0, 0, 1};

// --- 4. Mesh Settings ---
Mesh.CharacteristicLengthMin = 0.2;
Mesh.CharacteristicLengthMax = 0.2;