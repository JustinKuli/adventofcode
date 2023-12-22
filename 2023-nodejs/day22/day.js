#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let initialBricksByZ = {}

    lines.forEach((line, i) => {
        let pointStrs = line.split("~")
        let [x1,y1,z1] = pointStrs[0].split(",").map((n) => Number(n))
        let [x2,y2,z2] = pointStrs[1].split(",").map((n) => Number(n))

        // the smaller ones
        let x = (x1 < x2) ? x1 : x2
        let y = (y1 < y2) ? y1 : y2
        let z = (z1 < z2) ? z1 : z2

        let pts = [{x: x, y: y, z: z}]

        if (x1 === x2 && y1 === y2) {
            if (z === z1) {
                for (let zd=z+1; zd <= z2; zd++) {
                    pts.push({x: x, y: y, z: zd})
                }
            } else {
                for (let zd=z+1; zd <= z1; zd++) {
                    pts.push({x: x, y: y, z: zd})
                }
            }
        } else if (x1 === x2 && z1 === z2) {
            if (y === y1) {
                for (let yd=y+1; yd <= y2; yd++) {
                    pts.push({x: x, y: yd, z: z})
                }
            } else {
                for (let yd=y+1; yd <= y1; yd++) {
                    pts.push({x: x, y: yd, z: z})
                }
            }
        } else if (y1 === y2 && z1 === z2) {
            if (x === x1) {
                for (let xd=x+1; xd <= x2; xd++) {
                    pts.push({x: xd, y: y, z: z})
                }
            } else {
                for (let xd=x+1; xd <= x1; xd++) {
                    pts.push({x: xd, y: y, z: z})
                }
            }
        } else {
            console.log("examine line", i)
        }

        if (z in initialBricksByZ) {
            initialBricksByZ[z].push({id: i, pts: pts})
        } else {
            initialBricksByZ[z] = [{id: i, pts: pts}]
        }
    })

    let tops = {}
    let occupied = {}
    let loweredBricksById = {}

    // lower the bricks
    Object.keys(initialBricksByZ).sort((a,b) => a - b).forEach((key) => {
        initialBricksByZ[key].forEach((b) => {
            let lowestZ = 1
            b.pts.forEach((pt) => {
                let top = tops[xy(pt)]
                if (top && top >= lowestZ) {
                    lowestZ = top + 1
                }
            })

            let zDiff = b.pts[0].z - lowestZ
            b.pts.forEach((pt) => {
                pt.z -= zDiff
                occupied[xyz(pt)] = b.id

                const pos = xy(pt)
                if ((tops[pos] || 0) < pt.z) {
                    tops[pos] = pt.z
                }
            })

            loweredBricksById[b.id] = {pts: b.pts}
        })
    })

    // print(occupied)

    // identify how bricks interact with each other
    Object.keys(loweredBricksById).forEach((bid) => {
        let brick = loweredBricksById[bid]

        let needs = {}
        let helps = {}

        brick.pts.forEach((pt) => {
            const pos = xy(pt)

            const above = pos + "," + (pt.z+1)
            if (above in occupied) {
                helps[occupied[above]] = true
            }

            const below = pos + "," + (pt.z-1)
            if (below in occupied) {
                needs[occupied[below]] = true
            }
        })

        delete needs[bid]
        delete helps[bid]
        
        loweredBricksById[bid] = {
            pts: brick.pts,
            needs: Object.keys(needs),
            helps: Object.keys(helps)
        }
    })

    let count = 0
    let p2brickIDs = []

    // identify disintegrable bricks for part 1
    Object.keys(loweredBricksById).forEach((key) => {
        const brick = loweredBricksById[key]
        if (brick.helps.length === 0) {
            count++ // doesn't support any other bricks
        } else {
            let soleSupport = false
            brick.helps.forEach((bid) => {
                const supported = loweredBricksById[bid]
                if (supported.needs.length === 1) {
                    soleSupport = true
                }
            })

            if (!soleSupport) {
                count++
            } else {
                p2brickIDs.push(key)
            }
        }
    }) 

    console.log("Part One", count)

    let sum = 0

    p2brickIDs.forEach((id) => {
        // console.log("p2 with", id)
        let prevLength = 0

        let disintegrated = {}
        disintegrated[id] = true

        let unresolved = {}
        unresolved[id] = true

        while (prevLength !== Object.keys(disintegrated).length) {
            prevLength = Object.keys(disintegrated).length
            let newUnresolved = {}

            // console.log(unresolved)

            Object.keys(unresolved).forEach((brickID) => {
                let allResolved = true
                loweredBricksById[brickID].helps.forEach((considerID) => {
                    let allDisintegrated = true
                    loweredBricksById[considerID].needs.forEach((needID) => {
                        if (!(needID in disintegrated)) {
                            allDisintegrated = false
                            allResolved = false
                        }
                    })

                    if (allDisintegrated) {
                        if (!(considerID in disintegrated)) {
                            // console.log("disintegrating", considerID)
                            disintegrated[considerID] = true
                            newUnresolved[considerID] = true
                        }
                    }
                })

                if (!allResolved) {
                    newUnresolved[brickID] = true
                }
            })

            unresolved = newUnresolved
        }

        sum += prevLength - 1
    })

    console.log("Part Two", sum)
});

function xy(pt) {
    return pt.x + "," + pt.y
}

function xyz(pt) {
    return pt.x + "," + pt.y + "," + pt.z
}

function print(occupied) {
    for (let z = 1; z <= 8; z++) {
        console.log("z = ", z)
        
        for (let y = 0; y <= 2; y++) {
            let line = ""

            for (let x = 0; x <= 2; x++) {
                let pos = x + "," + y + "," + z
                if (pos in occupied) {
                    line += occupied[pos]
                } else {
                    line += "."
                }
            }

            console.log(line)
        }

        console.log()
    }
}
