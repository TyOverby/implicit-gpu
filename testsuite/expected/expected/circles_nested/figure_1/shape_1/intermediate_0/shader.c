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

  float x_0 = x_s - 10.136902;
  float y_1 = y_s - 210.1369;

  float circle_2;
  {
    float dx_3 = x_0 - 400;
    float dy_4 = y_1 - 0;
    circle_2 = sqrt(dx_3 * dx_3 + dy_4 * dy_4) - 52.034225;
  }

  float circle_5;
  {
    float dx_6 = x_0 - 300;
    float dy_7 = y_1 - 0;
    circle_5 = sqrt(dx_6 * dx_6 + dy_7 * dy_7) - 82.853546;
  }

  float circle_8;
  {
    float dx_9 = x_0 - 400;
    float dy_10 = y_1 - 0;
    circle_8 = sqrt(dx_9 * dx_9 + dy_10 * dy_10) - 52.034225;
  }

  float modulate_11 = circle_8 + -10;

  float not_12 = -modulate_11;

  float and_13 = max(circle_5, not_12);

  float or_14 = min(circle_2, and_13);

  float circle_15;
  {
    float dx_16 = x_0 - 240;
    float dy_17 = y_1 - 0;
    circle_15 = sqrt(dx_16 * dx_16 + dy_17 * dy_17) - 130.47241;
  }

  float circle_18;
  {
    float dx_19 = x_0 - 400;
    float dy_20 = y_1 - 0;
    circle_18 = sqrt(dx_19 * dx_19 + dy_20 * dy_20) - 52.034225;
  }

  float circle_21;
  {
    float dx_22 = x_0 - 300;
    float dy_23 = y_1 - 0;
    circle_21 = sqrt(dx_22 * dx_22 + dy_23 * dy_23) - 82.853546;
  }

  float circle_24;
  {
    float dx_25 = x_0 - 400;
    float dy_26 = y_1 - 0;
    circle_24 = sqrt(dx_25 * dx_25 + dy_26 * dy_26) - 52.034225;
  }

  float modulate_27 = circle_24 + -10;

  float not_28 = -modulate_27;

  float and_29 = max(circle_21, not_28);

  float or_30 = min(circle_18, and_29);

  float modulate_31 = or_30 + -10;

  float not_32 = -modulate_31;

  float and_33 = max(circle_15, not_32);

  float or_34 = min(or_14, and_33);

  float circle_35;
  {
    float dx_36 = x_0 - 200;
    float dy_37 = y_1 - 0;
    circle_35 = sqrt(dx_36 * dx_36 + dy_37 * dy_37) - 208.1369;
  }

  float circle_38;
  {
    float dx_39 = x_0 - 400;
    float dy_40 = y_1 - 0;
    circle_38 = sqrt(dx_39 * dx_39 + dy_40 * dy_40) - 52.034225;
  }

  float circle_41;
  {
    float dx_42 = x_0 - 300;
    float dy_43 = y_1 - 0;
    circle_41 = sqrt(dx_42 * dx_42 + dy_43 * dy_43) - 82.853546;
  }

  float circle_44;
  {
    float dx_45 = x_0 - 400;
    float dy_46 = y_1 - 0;
    circle_44 = sqrt(dx_45 * dx_45 + dy_46 * dy_46) - 52.034225;
  }

  float modulate_47 = circle_44 + -10;

  float not_48 = -modulate_47;

  float and_49 = max(circle_41, not_48);

  float or_50 = min(circle_38, and_49);

  float circle_51;
  {
    float dx_52 = x_0 - 240;
    float dy_53 = y_1 - 0;
    circle_51 = sqrt(dx_52 * dx_52 + dy_53 * dy_53) - 130.47241;
  }

  float circle_54;
  {
    float dx_55 = x_0 - 400;
    float dy_56 = y_1 - 0;
    circle_54 = sqrt(dx_55 * dx_55 + dy_56 * dy_56) - 52.034225;
  }

  float circle_57;
  {
    float dx_58 = x_0 - 300;
    float dy_59 = y_1 - 0;
    circle_57 = sqrt(dx_58 * dx_58 + dy_59 * dy_59) - 82.853546;
  }

  float circle_60;
  {
    float dx_61 = x_0 - 400;
    float dy_62 = y_1 - 0;
    circle_60 = sqrt(dx_61 * dx_61 + dy_62 * dy_62) - 52.034225;
  }

  float modulate_63 = circle_60 + -10;

  float not_64 = -modulate_63;

  float and_65 = max(circle_57, not_64);

  float or_66 = min(circle_54, and_65);

  float modulate_67 = or_66 + -10;

  float not_68 = -modulate_67;

  float and_69 = max(circle_51, not_68);

  float or_70 = min(or_50, and_69);

  float modulate_71 = or_70 + -10;

  float not_72 = -modulate_71;

  float and_73 = max(circle_35, not_72);

  float or_74 = min(or_34, and_73);

  buffer[pos] = or_74; 
}
