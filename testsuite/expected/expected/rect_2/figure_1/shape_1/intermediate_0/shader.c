float dist_to_line(float x, float y, float x1, float y1, float x2, float y2)
{
    float A = x - x1;
    float B = y - y1;
    float C = x2 - x1;
    float D = y2 - y1;

    float dot = A * C + B * D;
    float len_sq = C * C + D * D;
    float param = -1;

    if (len_sq != 0)
    {
        param = dot / len_sq;
    }

    float xx;
    float yy;

    if (param < 0)
    {
        xx = x1;
        yy = y1;
    }
    else if (param > 1)
    {
        xx = x2;
        yy = y2;
    }
    else
    {
        xx = x1 + param * C;
        yy = y1 + param * D;
    }

    float dx = x - xx;
    float dy = y - yy;

    return sqrt(dx * dx + dy * dy);
}

float position(float x, float y, float x1, float y1, float x2, float y2)
{
    return (x2 - x1) * (y - y1) - (y2 - y1) * (x - x1);
}

__kernel void apply(__global float* buffer, ulong width) {
  size_t x = get_global_id(0);
  size_t y = get_global_id(1);
  size_t pos = x + y * width;

  float x_s = (float) x;
  float y_s = (float) y;

  float x_0 = x_s - 0;
  float y_1 = y_s - 0;

  float rect_2 = INFINITY; float rect_2_sign = 0.0;
  {

    float temp_6 = dist_to_line(x_0, y_1, 2, 2, 12, 2);
    float temp_sign_7 = sign(position(x_0, y_1, 2, 2, 12, 2));
    if (fabs(temp_6) < fabs(rect_2)) {
        rect_2 = copysign(temp_6, temp_sign_7);
        rect_2_sign = temp_sign_7;
    }
    if (temp_6 == rect_2 && rect_2_sign != temp_sign_7) {
        rect_2 = copysign(rect_2, -1);
    }

    temp_6 = dist_to_line(x_0, y_1, 12, 2, 12, 12);
    temp_sign_7 = sign(position(x_0, y_1, 12, 2, 12, 12));
    if (fabs(temp_6) < fabs(rect_2)) {
        rect_2 = copysign(temp_6, temp_sign_7);
        rect_2_sign = temp_sign_7;
    }
    if (temp_6 == rect_2 && rect_2_sign != temp_sign_7) {
        rect_2 = copysign(rect_2, -1);
    }

    temp_6 = dist_to_line(x_0, y_1, 12, 12, 2, 12);
    temp_sign_7 = sign(position(x_0, y_1, 12, 12, 2, 12));
    if (fabs(temp_6) < fabs(rect_2)) {
        rect_2 = copysign(temp_6, temp_sign_7);
        rect_2_sign = temp_sign_7;
    }
    if (temp_6 == rect_2 && rect_2_sign != temp_sign_7) {
        rect_2 = copysign(rect_2, -1);
    }

    temp_6 = dist_to_line(x_0, y_1, 2, 12, 2, 2);
    temp_sign_7 = sign(position(x_0, y_1, 2, 12, 2, 2));
    if (fabs(temp_6) < fabs(rect_2)) {
        rect_2 = copysign(temp_6, temp_sign_7);
        rect_2_sign = temp_sign_7;
    }
    if (temp_6 == rect_2 && rect_2_sign != temp_sign_7) {
        rect_2 = copysign(rect_2, -1);
    }
    rect_2 = -rect_2;

  }

  buffer[pos] = rect_2; 
}
