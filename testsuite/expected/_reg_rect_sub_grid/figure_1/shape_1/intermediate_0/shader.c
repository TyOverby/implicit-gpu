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

  float x_0 = x_s - 7;
  float y_1 = y_s - 7;

  float rect_2 = INFINITY; float rect_2_sign = 0.0;
  {

    float temp_6 = dist_to_line(x_0, y_1, -5, -5, 75, -5);
    float temp_sign_7 = sign(position(x_0, y_1, -5, -5, 75, -5));
    if (fabs(temp_6) < fabs(rect_2)) {
        rect_2 = copysign(temp_6, temp_sign_7);
        rect_2_sign = temp_sign_7;
    }
    if (temp_6 == rect_2 && rect_2_sign != temp_sign_7) {
        rect_2 = copysign(rect_2, -1);
    }

    temp_6 = dist_to_line(x_0, y_1, 75, -5, 75, 75);
    temp_sign_7 = sign(position(x_0, y_1, 75, -5, 75, 75));
    if (fabs(temp_6) < fabs(rect_2)) {
        rect_2 = copysign(temp_6, temp_sign_7);
        rect_2_sign = temp_sign_7;
    }
    if (temp_6 == rect_2 && rect_2_sign != temp_sign_7) {
        rect_2 = copysign(rect_2, -1);
    }

    temp_6 = dist_to_line(x_0, y_1, 75, 75, -5, 75);
    temp_sign_7 = sign(position(x_0, y_1, 75, 75, -5, 75));
    if (fabs(temp_6) < fabs(rect_2)) {
        rect_2 = copysign(temp_6, temp_sign_7);
        rect_2_sign = temp_sign_7;
    }
    if (temp_6 == rect_2 && rect_2_sign != temp_sign_7) {
        rect_2 = copysign(rect_2, -1);
    }

    temp_6 = dist_to_line(x_0, y_1, -5, 75, -5, -5);
    temp_sign_7 = sign(position(x_0, y_1, -5, 75, -5, -5));
    if (fabs(temp_6) < fabs(rect_2)) {
        rect_2 = copysign(temp_6, temp_sign_7);
        rect_2_sign = temp_sign_7;
    }
    if (temp_6 == rect_2 && rect_2_sign != temp_sign_7) {
        rect_2 = copysign(rect_2, -1);
    }
    rect_2 = -rect_2;

  }

  float circle_8;
  {
    float dx_9 = x_0 - 0;
    float dy_10 = y_1 - 0;
    circle_8 = sqrt(dx_9 * dx_9 + dy_10 * dy_10) - 5;
  }

  float circle_11;
  {
    float dx_12 = x_0 - 0;
    float dy_13 = y_1 - 15;
    circle_11 = sqrt(dx_12 * dx_12 + dy_13 * dy_13) - 5;
  }

  float or_14 = min(circle_8, circle_11);

  float circle_15;
  {
    float dx_16 = x_0 - 0;
    float dy_17 = y_1 - 30;
    circle_15 = sqrt(dx_16 * dx_16 + dy_17 * dy_17) - 5;
  }

  float circle_18;
  {
    float dx_19 = x_0 - 0;
    float dy_20 = y_1 - 45;
    circle_18 = sqrt(dx_19 * dx_19 + dy_20 * dy_20) - 5;
  }

  float or_21 = min(circle_15, circle_18);

  float or_22 = min(or_14, or_21);

  float circle_23;
  {
    float dx_24 = x_0 - 15;
    float dy_25 = y_1 - 0;
    circle_23 = sqrt(dx_24 * dx_24 + dy_25 * dy_25) - 5;
  }

  float circle_26;
  {
    float dx_27 = x_0 - 15;
    float dy_28 = y_1 - 15;
    circle_26 = sqrt(dx_27 * dx_27 + dy_28 * dy_28) - 5;
  }

  float or_29 = min(circle_23, circle_26);

  float circle_30;
  {
    float dx_31 = x_0 - 15;
    float dy_32 = y_1 - 30;
    circle_30 = sqrt(dx_31 * dx_31 + dy_32 * dy_32) - 5;
  }

  float circle_33;
  {
    float dx_34 = x_0 - 15;
    float dy_35 = y_1 - 45;
    circle_33 = sqrt(dx_34 * dx_34 + dy_35 * dy_35) - 5;
  }

  float or_36 = min(circle_30, circle_33);

  float or_37 = min(or_29, or_36);

  float or_38 = min(or_22, or_37);

  float circle_39;
  {
    float dx_40 = x_0 - 30;
    float dy_41 = y_1 - 0;
    circle_39 = sqrt(dx_40 * dx_40 + dy_41 * dy_41) - 5;
  }

  float circle_42;
  {
    float dx_43 = x_0 - 30;
    float dy_44 = y_1 - 15;
    circle_42 = sqrt(dx_43 * dx_43 + dy_44 * dy_44) - 5;
  }

  float or_45 = min(circle_39, circle_42);

  float circle_46;
  {
    float dx_47 = x_0 - 30;
    float dy_48 = y_1 - 30;
    circle_46 = sqrt(dx_47 * dx_47 + dy_48 * dy_48) - 5;
  }

  float circle_49;
  {
    float dx_50 = x_0 - 30;
    float dy_51 = y_1 - 45;
    circle_49 = sqrt(dx_50 * dx_50 + dy_51 * dy_51) - 5;
  }

  float or_52 = min(circle_46, circle_49);

  float or_53 = min(or_45, or_52);

  float circle_54;
  {
    float dx_55 = x_0 - 45;
    float dy_56 = y_1 - 0;
    circle_54 = sqrt(dx_55 * dx_55 + dy_56 * dy_56) - 5;
  }

  float circle_57;
  {
    float dx_58 = x_0 - 45;
    float dy_59 = y_1 - 15;
    circle_57 = sqrt(dx_58 * dx_58 + dy_59 * dy_59) - 5;
  }

  float or_60 = min(circle_54, circle_57);

  float circle_61;
  {
    float dx_62 = x_0 - 45;
    float dy_63 = y_1 - 30;
    circle_61 = sqrt(dx_62 * dx_62 + dy_63 * dy_63) - 5;
  }

  float circle_64;
  {
    float dx_65 = x_0 - 45;
    float dy_66 = y_1 - 45;
    circle_64 = sqrt(dx_65 * dx_65 + dy_66 * dy_66) - 5;
  }

  float or_67 = min(circle_61, circle_64);

  float or_68 = min(or_60, or_67);

  float or_69 = min(or_53, or_68);

  float or_70 = min(or_38, or_69);

  float not_71 = -or_70;

  float and_72 = max(rect_2, not_71);

  buffer[pos] = and_72; 
}
