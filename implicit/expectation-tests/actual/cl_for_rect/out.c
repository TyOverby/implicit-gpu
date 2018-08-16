
// Rect _rect_0
float _rect_0 = INFINITY;
_rect_0 = min(_rect_0, dist_to_line(x, y, 0, 5, 10, 5))
_rect_0 = min(_rect_0, dist_to_line(x, y, 10, 5, 10, 25))
_rect_0 = min(_rect_0, dist_to_line(x, y, 10, 25, 0, 25))
_rect_0 = min(_rect_0, dist_to_line(x, y, 0, 25, 0, 5))
if (x > 0 && y > 5 && x < (0 + 10) && y < (5 + 20))
_rect_0 = -_rect_0;
// End Rect _rect_0
