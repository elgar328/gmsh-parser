SetFactory("OpenCASCADE");
Box(1) = {-0.5, -0.5, -0.5, 1, 1, 1};
Physical Surface("top", 13) = {4};
Physical Surface("bottom", 14) = {3};
Physical Volume("volume1", 15) = {1};
Mesh 3;
Save "box.msh";
