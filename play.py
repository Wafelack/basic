import json
from urllib.request import urlopen


def play(code):
    file = open('code.rs', 'w')
    for i in range(1, len(code)):
        file.write(code[i].replace('`', ''))
    file.close()
    bfile = open('code.rs', 'rb')

    json_data = dict(channel="stable", code=bfile.read().decode("utf-8"), crateType="bin", mode="debug", tests=False,
                     assemblyFlavor="att",
                     demangleAssembly="demangle", target="ast", execute=True)
    data: bytes = json.dumps(json_data).encode('utf-8')
    print(type(data))
    url = "https://play.rust-lang.org/execute"
    response = urlopen(url, data)
    ret = json.loads(response.read())
    for key, value in ret.items():
        ret[key] = str(value).rstrip()
    return ret["stderr"], ret["stdout"]
