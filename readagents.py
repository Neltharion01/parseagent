def cut(string, sep):
    before, sep, after = string.partition(sep);
    return before, after;
def parse(s):
    ip, s = cut(s, " ");
    minus, s = cut(s, " ");
    user, s = cut(s, " [");
    time, s = cut(s, "] \"");
    req, s = cut(s, "\" ");
    code, s = cut(s, " ");
    length, s = cut(s, " \"");
    referer, s = cut(s, "\" \"");
    useragent, s = cut(s, "\" \"");
    forwardedfor, s = cut(s, "\" ");
    assert len(s) == 0;
    return useragent;

f = open("/var/log/nginx/access.log", "r");

for line in f: print(parse(line));
f.close();
