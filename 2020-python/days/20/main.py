#!/usr/bin/env python3

import sys

data = []
with open('formatted.txt', 'r') as file:
    for line in file:
        data.append(line[:-1].split(",")) # Strip the newline char!

class Tile:
    def __init__(self, inp):
        self.id = int(inp[0].split(" ")[1][:-1])
        self.top = inp[1]
        self.bottom = inp[-1]
        self.left = ""
        self.right = ""
        for line in inp[1:]:
            self.left += line[0]
            self.right += line[-1]
        self.interior = []
        for line in inp[2:-1]:
            self.interior.append(line[1:-1])
        self.connections = []
    
    def __repr__(self):
        ans = "=== TILE ===\n"
        ans += "id: {}\n".format(self.id)
        ans += "connections: {}\n".format(self.connections)
        # ans += "top: {}\n".format(self.top)
        # ans += "bottom: {}\n".format(self.bottom)
        # ans += "left: {}\n".format(self.left)
        # ans += "right: {}\n".format(self.right)
        ans += "interior:\n"
        for line in self.interior:
            ans += "{}\n".format(line)
        ans += "=== END ==="
        return ans
    
    def flip(self):
        self.top = self.top[::-1]
        self.bottom = self.bottom[::-1]
        new_interior = []
        for line in self.interior:
            new_interior.append(line[::-1])
        self.interior = new_interior
        hold = self.right
        self.right = self.left
        self.left = hold
    
    def rotate(self):
        hold = self.left
        self.left = self.bottom
        self.bottom = self.right[::-1]
        self.right = self.top
        self.top = hold[::-1]

        new_interior = []
        for i in range(len(self.interior)):
            line = ""
            for j in range(len(self.interior)):
                line += self.interior[-j-1][i]
            new_interior.append(line)
        self.interior = new_interior
    
    def _simple_connect(self, other):
        if self.top == other.bottom:
            return "top"
        if self.right == other.left:
            return "right"
        if self.bottom == other.top:
            return "bottom"
        if self.left == other.right:
            return "left"
        return ""
    
    def _rotate_connect(self, other):
        for r in [0,1,2,3]:
            ans = self._simple_connect(other)
            if ans != "":
                return ans
            other.rotate()
        return ""
    
    def connect(self, other):
        ans = self._rotate_connect(other)
        if ans != "":
            if other.id not in self.connections:
                self.connections.append(other.id)
            return ans
        
        other.flip()
        ans = self._rotate_connect(other)
        if ans != "":
            if other.id not in self.connections:
                self.connections.append(other.id)
            return ans

        return ""

def get_tiles():
    tiles = dict()
    for d in data:
        t = Tile(d)
        tiles[t.id] = t
    return tiles

def populate_connections(tiles):
    for t1 in tiles:
        for t2 in tiles:
            if t1 == t2:
                continue
            tiles[t1].connect(tiles[t2])

tiles = get_tiles()
populate_connections(tiles)

def part_one():
    corners = [t for t in tiles if len(tiles[t].connections) == 2]
    
    prod = 1
    for c in corners:
        prod *= c

    print("part one answer:", prod)

class Picture:
    def __init__(self, inp):
        self.interior = inp

    def flip(self):
        new_interior = []
        for line in self.interior:
            new_interior.append(line[::-1])
        self.interior = new_interior
    
    def rotate(self):
        new_interior = []
        for i in range(len(self.interior)):
            line = ""
            for j in range(len(self.interior)):
                line += self.interior[-j-1][i]
            new_interior.append(line)
        self.interior = new_interior
    
    def _mon_at(self, row, col, mon):
        pic_row = self.interior[row]
        for n, c in enumerate(mon):
            # print(pic_row, len(pic_row), col, n)
            if c == "#" and pic_row[col+n] != "#":
                return False
        return True
                
    def _has_sea_monster(self):
        mon0 = "                  # "
        mon1 = "#    ##    ##    ###"
        mon2 = " #  #  #  #  #  #   "
        for row in range(len(self.interior) - 2 ):
            for col in range(len(self.interior[0]) - (len(mon1) - 1)):
                if self._mon_at(row, col, mon0):
                    if self._mon_at(row+1, col, mon1):
                        if self._mon_at(row+2, col, mon2):
                            return (row, col)
        return False

    def transform_until_monster(self):
        for r in range(0,4):
            if self._has_sea_monster():
                return
            # print("Rotating to find monster")
            self.rotate()
        
        self.flip()
        # print("Flipping to find monster")
        for r in range(0,4):
            if self._has_sea_monster():
                return
            # print("Rotating to find monster")
            self.rotate()
        
        print("Could not transform to find a sea monster!")
        sys.exit(1)
    
    def _clear_pixel(self, row, col):
        self.interior[row] = self.interior[row][0:col] + "O" + self.interior[row][col+1:]

    def remove_monsters(self):
        mon = []
        mon.append("                  # ")
        mon.append("#    ##    ##    ###")
        mon.append(" #  #  #  #  #  #   ")

        mon_pos = self._has_sea_monster()
        while mon_pos:
            # print("== found a mon ==")
            for j, m in enumerate(mon):
                for i, c in enumerate(m):
                    if c == "#":
                        self._clear_pixel(mon_pos[0]+j, mon_pos[1]+i)
            mon_pos = self._has_sea_monster()

def combine_images():
    corners = [t for t in tiles if len(tiles[t].connections) == 2]
    edges = [t for t in tiles if len(tiles[t].connections) == 3]
    interiors =[t for t in tiles if len(tiles[t].connections) == 4]

    rows = []
    row = []
    placed = []

    # Pick first corner arbitrarily
    cur = tiles[corners[0]]

    # Need to carefully orient the first piece so it's in the top left
    poss = cur.connections
    tries = 1
    while not (cur.connect(tiles[poss[0]]) == "right" and cur.connect(tiles[poss[1]]) == "bottom"):
        cur.rotate()
        if tries == 4:
            cur.flip()
        tries += 1
    
    placed.append(cur.id)
    # print("placed c", cur.id)
    row.append(cur.id)

    # Put the first edge of the top row
    nxt = tiles[cur.connections[0]]
    cur.connect(nxt)
    placed.append(nxt.id)
    # print("placed e", nxt.id)
    row.append(nxt.id)

    # Put the remaining edges along the top
    while True:
        cur = nxt
        poss = [t for t in cur.connections if t in edges and t not in placed]
        if len(poss) != 1:
            break
        nxt = tiles[poss[0]]
        cur.connect(nxt)
        placed.append(nxt.id)
        # print("placed e", nxt.id)
        row.append(nxt.id)

    # Put the other top corner.
    poss = [t for t in cur.connections if t in corners and t not in placed]
    nxt = tiles[poss[0]]
    cur.connect(nxt)
    placed.append(nxt.id)
    # print("placed c", nxt.id)
    row.append(nxt.id)

    # print("===== Top Row Complete =====")
    rows.append(row)

    # Do the rest of the picture
    while len([t for t in tiles[rows[-1][0]].connections if t not in placed]) == 1:
        row = []
        for i in range(len(rows[0])):
            cur = tiles[rows[-1][i]]
            poss = [t for t in cur.connections if t not in placed]
            if len(poss) != 1:
                break
            nxt = tiles[poss[0]]
            cur.connect(nxt)
            placed.append(nxt.id)
            # print("placed", nxt.id)
            row.append(nxt.id)
        
        # print("===== Row Complete =====")
        rows.append(row)
    
    # for r in rows:
    #     print(r)
    
    full_pic = []

    for row in rows:
        for i in range(len(tiles[row[0]].interior)):
            pic_row = ""
            for tile in row:
                pic_row += tiles[tile].interior[i]
            # print(pic_row)
            full_pic.append(pic_row)

    return Picture(full_pic)

def part_two():
    pic = combine_images()
    pic.transform_until_monster()
    pic.remove_monsters()

    count = 0
    for r in pic.interior:
        # print(r)
        for c in r:
            if c == "#":
                count += 1

    print("part two answer:", count)

part_one()
part_two()
