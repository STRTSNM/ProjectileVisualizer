import math
import os
import time

def draw(c):
    mx = max(c, key=lambda q: q[0])[0]
    my = max(c, key=lambda q: q[1])[1]

    for y in range(my, -1, -1):  # Invert Y-axis coordinates
        line = ""
        for x in range(mx + 1):
            if (x, y) in c:
                line += "$"
            else:
                line += " "
        print(line)

def projectile():
    u = 20  # Initial velocity (m/s)
    b = 37  # Launch angle (degrees)
    g = 9.81  # Acceleration due to gravity (m/s^2)
    accuracy=0.05
    uy = u * math.sin(math.radians(b))
    ux = u * math.cos(math.radians(b))

    t = 0
    x, y = 0, 50
    r=2*ux*uy/g
    tp=2*uy/g
    hm=(uy**2)/(g**2)

    trajectory = []
    while y >= 0: 
        t += accuracy
        x = ux * t
        y = uy * t - 0.5 * g * t**2

        trajectory.append((int(x*60/r),int(y*60/r))) #60/r is the factor to be multiplied to bring the path under 30 horizontal characters

        draw(trajectory)
        time.sleep(0.05)
        os.system("cls" if os.name == "nt" else "clear")
    draw(trajectory[:-1])
    print("\n"*5)
    print("RANGE : "+str(r),"TIME PERIOD : "+str(tp),"MAXIMUM HEIGHT : "+str(hm), sep="\n")
    print("*Path calculated with an accuracy of "+str(accuracy)+"*")

projectile()
