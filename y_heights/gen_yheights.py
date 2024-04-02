import random as rn


def gen_yheights():
    with open("y-heigts.scn","+a") as f:
        for i in range(20):
            for j in range(20):
                y = rn.random()
                y*=100
                y+=1
                f.writelines("{}\n".format(y))



if __name__ == "__main__":
    gen_yheights()