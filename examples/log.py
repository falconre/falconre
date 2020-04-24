import colored 

def info(msg):
    print("[.] {}".format(msg))

def alert(msg):
    print("{}[*] {}{}".format(colored.fg(3), msg, colored.attr(0)))

def warn(msg):
    alert(msg)

def blue(msg):
    print("{}[.] {}{}".format(colored.fg(4), msg, colored.attr(0)))

def cyan(msg):
    print("{}[.] {}{}".format(colored.fg(6), msg, colored.attr(0)))

def magenta(msg):
    print("{}[.] {}{}".format(colored.fg(5), msg, colored.attr(0)))

def error(msg):
    print("{}[-] {}{}".format(colored.fg(9), msg, colored.attr(0)))

def success(msg):
    print("{}[+] {}{}".format(colored.fg(2), msg, colored.attr(0)))

def green(msg):
    success(msg)