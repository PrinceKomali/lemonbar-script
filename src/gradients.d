import std.conv, std.format, std.string;

struct Color
{
    int r;
    int g;
    int b;
}

Color to_color(int r, int g, int b)
{
    Color c;
    c.r = r;
    c.g = g;
    c.b = b;
    return c;
}

extern (C) Color rg_gradient(int o)
{
    if (o == 50)
    {
        return to_color(255, 255, 0);
    }
    float r = o < 50 ? (to!float(o) / 50.0) * 255.0 : 255.0;
    float g = o > 50 ? 255.0 - ((to!float(o) - 50.0) / 50.0) * 255.0 : 255.0;
    return to_color(to!int(r), to!int(g), 0);
}

extern (C) Color br_gradient(int o)
{
    if (o == 50)
    {
        return to_color(255, 0, 255);
    }
    float r = o < 50 ? (to!float(o) / 50.0) * 255.0 : 255.0;
    float b = o > 50 ? 255.0 - ((to!float(o) - 50.0) / 50.0) * 255.0 : 255.0;
    return to_color(to!int(r), 0, to!int(b));
}
