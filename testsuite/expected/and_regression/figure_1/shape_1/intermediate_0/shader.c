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

  float x_0 = x_s - -8;
  float y_1 = y_s - -8;

  float circle_2;
  {
    float dx_3 = x_0 - 20;
    float dy_4 = y_1 - 20;
    circle_2 = sqrt(dx_3 * dx_3 + dy_4 * dy_4) - 10;
  }

  float rect_5 = INFINITY; float rect_5_sign = 0.0;
  {

    float temp_9 = dist_to_line(x_0, y_1, 0, 0, 50, 0);
    float temp_sign_10 = sign(position(x_0, y_1, 0, 0, 50, 0));
    if (fabs(temp_9) < fabs(rect_5)) {
        rect_5 = copysign(temp_9, temp_sign_10);
        rect_5_sign = temp_sign_10;
    }
    if (temp_9 == rect_5 && rect_5_sign != temp_sign_10) {
        rect_5 = copysign(rect_5, -1);
    }

    temp_9 = dist_to_line(x_0, y_1, 50, 0, 50, 50);
    temp_sign_10 = sign(position(x_0, y_1, 50, 0, 50, 50));
    if (fabs(temp_9) < fabs(rect_5)) {
        rect_5 = copysign(temp_9, temp_sign_10);
        rect_5_sign = temp_sign_10;
    }
    if (temp_9 == rect_5 && rect_5_sign != temp_sign_10) {
        rect_5 = copysign(rect_5, -1);
    }

    temp_9 = dist_to_line(x_0, y_1, 50, 50, 0, 50);
    temp_sign_10 = sign(position(x_0, y_1, 50, 50, 0, 50));
    if (fabs(temp_9) < fabs(rect_5)) {
        rect_5 = copysign(temp_9, temp_sign_10);
        rect_5_sign = temp_sign_10;
    }
    if (temp_9 == rect_5 && rect_5_sign != temp_sign_10) {
        rect_5 = copysign(rect_5, -1);
    }

    temp_9 = dist_to_line(x_0, y_1, 0, 50, 0, 0);
    temp_sign_10 = sign(position(x_0, y_1, 0, 50, 0, 0));
    if (fabs(temp_9) < fabs(rect_5)) {
        rect_5 = copysign(temp_9, temp_sign_10);
        rect_5_sign = temp_sign_10;
    }
    if (temp_9 == rect_5 && rect_5_sign != temp_sign_10) {
        rect_5 = copysign(rect_5, -1);
    }
    rect_5 = -rect_5;

  }

  float and_11 = max(circle_2, rect_5);

  buffer[pos] = and_11; 
}
