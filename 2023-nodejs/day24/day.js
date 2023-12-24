#!/usr/bin/env node

const fs = require('node:fs');

fs.readFile('./sample.txt', (err, data) => {
// fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let stones = []

    lines.forEach((line) => {
        let [pos, velocity] = line.split(" @ ")
        let [px, py, pz]  = pos.split(", ").map((n) => Number(n))
        let [vx, vy, vz]  = velocity.split(", ").map((n) => Number(n))
        stones.push({
            px: px, py: py, pz: pz,
            vx: vx, vy: vy, vz: vz,
            mxy: vy/vx
        })
    })

    let crossings = 0

    stones.forEach((s1, i) => {
        for (let j = i+1; j < stones.length; j++) {
            let s2 = stones[j]
            // if (futureCrossIn(s1, s2, 7, 27)) { // sample
            if (futureCrossIn(s1, s2, 200000000000000, 400000000000000)) { // real
                crossings++
            }
        }
    })

    console.log("Part One", crossings)

    let s1 = stones[1]
    let s2 = stones[2]
    let s3 = stones[3]
    let s4 = stones[4]

    let s1s2cross = {
        i: (s1.vy*s2.vz - s1.vz*s2.vy),
        j: -(s1.vx*s2.vz - s1.vz*s2.vx),
        k: (s1.vx*s2.vy - s1.vy*s2.vx)
    }

    let s3s4cross = {
        i: (s3.vy*s4.vz - s3.vz*s4.vy),
        j: -(s3.vx*s4.vz - s3.vz*s4.vx),
        k: (s3.vx*s4.vy - s3.vy*s4.vx)
    }

    // s1 s2 plane in ax + by + cz + d = 0 form
    let a1 = s1s2cross.i
    let b1 = s1s2cross.j
    let c1 = s1s2cross.k
    let d1 = -(s1s2cross.i*s1.px) - (s1s2cross.j*s1.py) - (s1s2cross.k*s1.pz)

    // s3 s4 plane in ax + by + cz + d = 0 form
    let a2 = s3s4cross.i
    let b2 = s3s4cross.j
    let c2 = s3s4cross.k
    let d2 = -(s3s4cross.i*s3.px) - (s3s4cross.j*s3.py) - (s3s4cross.k*s3.pz)

    // parameterized eqn for the intersection of the planes
    // ... hope I got this math right...
    let x_m = (b2/c2 - b1/c1)/(a1/c1 - a2/c2)
    let x_b = (d2/c2 - d1/c1)/(a1/c1 - a2/c2)

    // let x_m = (b2*c1 - b1*c2)/(a1*c2 - a2*c1)
    // let x_b = (c1*d2 - c2*d1)/(a1*c2 - a2*c1)
    // let y_m = 1
    // let y_b = 0
    // let z_m = (a2*b1 - a1*b2)/(a1*c2 - a2*c1)
    // let z_b = (c1*d2 - c2*d1)/(a1*c2 - a2*c1)

    // find two times and places the rock must be
    let t1 = (s1.px - x_m*s1.py - x_b) / (x_m*s1.vy - s1.vx)

    // let t1 = (s1.px - x_b - x_m*s1.py) / (x_m*s1.vy - s1.vx)
    let x1 = s1.px + s1.vx*t1
    let y1 = s1.py + s1.vy*t1
    let z1 = s1.pz + s1.vz*t1

    let t2 = (s2.px - x_b - x_m*s2.py) / (x_m*s2.vy - s2.vx)
    let x2 = s2.px + s2.vx*t2
    let y2 = s2.py + s2.vy*t2
    let z2 = s2.pz + s2.vz*t2

    console.log(t1, t2)

    // use those to adjust the parameterization
    let x_tm = (x2 - x1)/(t2 - t1)
    let x_tb = x1 - x_tm*t1
    let y_tm = (y2 - y1)/(t2 - t1)
    let y_tb = y1 - y_tm*t1
    let z_tm = (z2 - z1)/(t2 - t1)
    let z_tb = z1 - z_tm*t1

    console.log(x_tb + y_tb + z_tb)
}); // 782240311123103 is too high

function futureCrossIn(s1, s2, min, max) {
    // check if they're parallel
    if (s1.mxy === s2.mxy) {
        return false
    }

    let xCross = xOfCross(s1, s2)
    
    // check if xCross is within the bounds
    if (xCross < min || xCross > max) {
        return false
    }

    // check that xCross is in the future for s1
    // could do with a divide, but comparisons might be computationally faster?
    if (!(((xCross - s1.px > 0) && (s1.vx > 0)) || ((xCross - s1.px < 0) && (s1.vx < 0)))) {
        return false
    }

    // same for s2
    if (!(((xCross - s2.px > 0) && (s2.vx > 0)) || ((xCross - s2.px < 0) && (s2.vx < 0)))) {
        return false
    }

    // finally, find yCross and check if it is in the bounds
    let yCross = s1.mxy*(xCross - s1.px) + s1.py
    if (yCross < min || yCross > max) {
        return false
    }

    return true
}

function xOfCross(s1, s2) {
    return (s1.mxy*s1.px - s2.mxy*s2.px + s2.py - s1.py) / (s1.mxy - s2.mxy)
}
