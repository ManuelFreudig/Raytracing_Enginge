GAUSSIAN_BLUR_5 = [[0.003,0.013,0.022,0.13,0.003],
                   [0.013,0.06,0.098,0.06,0.013],
                   [0.022,0.098,0.162,0.098,0.022],
                   [0.013,0.06,0.098,0.06,0.013],
                   [0.003,0.013,0.022,0.13,0.003]]

GAUSSIAN_BLUR_3 = [[1/16,2/16,1/16],
                   [2/16,4/16,2/16],
                   [1/16,2/16,1/16]]

EDGE_DETECTION = [[0,-1/4,0],
                   [-1/4,0.25,-1/4],
                   [0,-1/4,0]]


def antialysing(file,matrix):
    with open(file,'r') as f:
        l1 = f.readline()
        l2 = f.readline()
        width, height = list(map(int,l2.split()))
        print(width, height)
        l3 = f.readline()
        image = []
        for i in range(height):
            row = []
            for j in range(width):
                row.append(list(map(float,f.readline().split())))
            image.append(row)
        print(image)
    file = file.split(".")[0]+"_a."+file.split(".")[1]
    with open(file,'w') as f:
        f.writelines([l1,l2,l3])
        for i in range(height):
            for j in range(width):
                color = [0,0,0]
                for x in range(len(matrix)):
                    for y in range(len(matrix[0])):
                        i_x = abs(i+y-2)
                        i_y = abs(j+x-2)
                        if i_x >= height:
                            i_x = height-i_x
                        if i_y >= width:
                            i_y = width-i_y
                        if matrix[x][y] < 0:
                            color[0] += image[i_x][i_y][1]*abs(matrix[x][y]) + image[i_x][i_y][2]*abs(matrix[x][y])
                            color[1] += image[i_x][i_y][0]*abs(matrix[x][y]) + image[i_x][i_y][2]*abs(matrix[x][y])                        
                            color[2] += image[i_x][i_y][1]*abs(matrix[x][y]) + image[i_x][i_y][0]*abs(matrix[x][y])
                        else:
                            color[0] += image[i_x][i_y][0]*matrix[x][y]
                            color[1] += image[i_x][i_y][1]*matrix[x][y]
                            color[2] += image[i_x][i_y][2]*matrix[x][y]
                f.writelines("{r} {g} {b}\n".format(r = int(color[0]),g = int(color[1]),b = int(color[2])))





if __name__ == "__main__":
    antialysing("images\Sphere_hittable_list.ppm",GAUSSIAN_BLUR_3)


