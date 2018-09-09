float2 dist_to_line_comp(float x, float y, float x1, float y1, float x2, float y2)
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

    float pos = (x2 - x1) * (y - y1) - (y2 - y1) * (x - x1);
    if (pos == 0) {
        pos = -1.0;
    }

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

    return (float2)(sqrt(dx * dx + dy * dy), pos);
}

float dist_to_line(float x, float y, float x1, float y1, float x2, float y2)
{
    return dist_to_line_comp(x, y, x1, y1, x2, y2).x;
}

float position(float x, float y, float x1, float y1, float x2, float y2)
{
    return dist_to_line_comp(x, y, x1, y1, x2, y2).y;
}
