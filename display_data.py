import matplotlib.pyplot as plt

if __name__ == "__main__":
    with open('./data.csv') as f:
        data = f.read()
        pairs = data.split('\n')
        xs = []
        ys = []
        for s in pairs:
            try:
                x, y = s.split(',')
                xs.append(int(x))
                ys.append(int(y))
            except:
                pass
        plt.scatter(xs, ys)
        plt.plot(xs, ys)
        plt.show()

