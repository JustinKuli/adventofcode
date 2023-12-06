#!/usr/bin/env node

const fs = require('node:fs');

// Returns three ranges: the first is the intersection of the two inputs,
// the second is the part of the first input which is not in the intersection,
// and the third is the part of the second input which is not in the intersection.
//
// Input ranges should have `start` and `len` fields.
function rangeOp(r1, r2) {
    const r1_end = r1.start + r1.len - 1
    const r2_end = r2.start + r2.len - 1

    const sortedPts = [r1.start, r1_end, r2.start, r2_end].sort((a, b) => a - b)

    // all one point
    if (sortedPts[0] === sortedPts[1] && 
        sortedPts[1] === sortedPts[2] && 
        sortedPts[2] === sortedPts[3]) {
            return [
                {start: sortedPts[0], len: 1},
                false,
                false
            ]
    }

    // three points together on the left
    if (sortedPts[0] === sortedPts[1] && sortedPts[1] === sortedPts[2]) {
        if (r1.len > 1) {
            return [
                {...r2},
                {start: r1.start + 1, len: r1.len - 1},
                false
            ]
        } else {
            return [
                {...r1},
                false,
                {start: r2.start + 1, len: r2.len - 1}    
            ]
        }
    }

    // three points together on the right
    if (sortedPts[1] === sortedPts[2] && sortedPts[2] === sortedPts[3]) {
        if (r1.len > 1) {
            return [
                {...r2},
                {start: r1.start, len: r1.len - 1},
                false
            ]
        } else {
            return [
                {...r1},
                false,
                {start: r2.start, len: r2.len - 1}
            ]
        }
    }

    // two distinct points
    if (sortedPts[0] === sortedPts[1] && sortedPts[2] === sortedPts[3]) {
        // Complete overlap
        if (r1.len > 1) {
            return [
                {...r1},
                false,
                false
            ]
        } else { // No overlap of 2 ranges with length 1
            return [
                false,
                {...r1},
                {...r2}
            ]
        }
    }

    // r1 is basically entirely before r2
    if (r1.start === sortedPts[0] && r1_end === sortedPts[1]) {
        // r1 is fully before r2, with absolutely no overlap
        if (sortedPts[1] !== sortedPts[2]) {
            return [
                false, 
                {...r1},
                {...r2}
            ]
        } else { // the intersect is just sortedPts[1]
            return [
                {start: sortedPts[1], len: 1},
                {start: r1.start, len: r1.len - 1},
                {start: r2.start + 1, len: r2.len - 1}
            ]
        }
    }

    // r2 is basically entirely before r1
    if (r2.start === sortedPts[0] && r2_end === sortedPts[1]) {
        // r2 is fully before r1, with absolutely no overlap
        if (sortedPts[1] !== sortedPts[2]) {
            return [
                false,
                {...r1},
                {...r2}
            ]
        } else { // the intersect is just sortedPts[1]
            return [
                {start: sortedPts[1], len: 1},
                {start: r1.start + 1, len: r1.len - 1},
                {start: r2.start, len: r2.len - 1}
            ]
        }
    }

    if (sortedPts[0] === sortedPts[1] && sortedPts[0] === r1.start && sortedPts[0] == r2.start) {
        if (r2.len > r1.len) {
            // r1 and r2 start together, but r2 is longer
            return [
                {...r1},
                false,
                {start: r1_end + 1, len: r2.len - r1.len}
            ]
        } else {
            // r1 and r2 start togeher, but r1 is longer
            return [
                {...r2},
                {start: r2_end + 1, len: r1.len - r2.len},
                false
            ]
        }
    }

    if (sortedPts[2] === sortedPts[3] && sortedPts[2] === r1_end && sortedPts[2] === r2_end) {
        if (r1.len < r2.len) {
            // r1 and r2 end together, and r1 is shorter
            return [
                {...r1},
                false,
                {start: r2.start, len: r2.len - r1.len}
            ]
        } else {
            // r1 and r2 end together, and r2 is shorter
            return [
                {...r2},
                {start: r1.start, len: r1.len - r2.len},
                false
            ]
        }
    }

    // from here on, all points are distinct
    // and I've already covered the cases where there is no overlap

    if (sortedPts[0] === r1.start) {
        let overlap_len = sortedPts[2] - sortedPts[1] + 1
        return [
            {start: sortedPts[1], len: overlap_len},
            {start: r1.start, len: r1.len - overlap_len},
            {start: sortedPts[2] + 1, len: r2.len - overlap_len}
        ]
    } else {
        let overlap_len = sortedPts[2] - sortedPts[1] + 1
        return [
            {start: sortedPts[1], len: overlap_len},
            {start: sortedPts[2] + 1, len: r1.len - overlap_len},
            {start: r2.start, len: r2.len - overlap_len}
        ]
    }

    console.log("GOT TO END INCORRECTLY?")
}

fs.readFile('./sample.txt', (err, data) => {
// fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let seeds = []
    let mapsets = []
    let mapsetidx = -1

    lines.forEach((line, i) => {
        if (i == 0) {
            line.split(" ").forEach((s, si, ss) => {
                // Part 1
                if (Number(s)) {
                    seeds.push({
                        start: Number(ss[si]),
                        len: 1
                    })
                }

                // Part 2
                // if (si%2 === 1) {
                //     seeds.push({
                //         start: Number(ss[si]),
                //         len: Number(ss[si+1])
                //     })
                // }
            })
        } else if (line.length > 1) {
            const parts = line.split(" ")
            if (parts[1] === "map:") {
                mapsetidx++
                mapsets.push([])
            } else { // reading numbers for mapsets[mapsetidx]
                mapsets[mapsetidx].push({
                    delta: Number(parts[0]) - Number(parts[1]),
                    start: Number(parts[1]),
                    len: Number(parts[2])
                })
            }
        }
    })

    mapsets.forEach((mapset) => {
        let mappedseeds = []
        let nonmappedseeds = []
        mapset.forEach((mapping) => {
            nonmappedseeds = []
            seeds.forEach((s) => {
                [overlap, seed_remnant, mapping_remnant] = rangeOp(s, mapping)
                if (!overlap) {
                    nonmappedseeds.push(s)
                    if (s.len != 1) {
                        console.log("pushed non 1 len nonmapped seed")
                        console.log("mapping", mapping)
                        console.log("s", s)
                        console.log("nonmappedseeds", nonmappedseeds)
                        console.log("mappedseeds", mappedseeds)
                        console.log("seeds", seeds)

                        process.exit(1)
                    }
                    return // no overlap between seed and mapping
                }

                overlap.start += mapping.delta
                mappedseeds.push(overlap)
                if (overlap.len != 1) {
                    console.log("pushed non 1 len mapped seed")
                    console.log("mapping", mapping)
                    console.log("overlap", overlap)
                    console.log("nonmappedseeds", nonmappedseeds)
                    console.log("mappedseeds", mappedseeds)
                    console.log("seeds", seeds)

                    process.exit(1)
                }

                if (seed_remnant) {
                    nonmappedseeds.push(seed_remnant)
                    if (seed_remnant.len != 1) {
                        console.log("pushed non 1 len seed_remnant")
                        console.log("mapping", mapping)
                        console.log("seed_remnant", seed_remnant)
                        console.log("nonmappedseeds", nonmappedseeds)
                        console.log("mappedseeds", mappedseeds)
                        console.log("seeds", seeds)

                        process.exit(1)
                    }
    
                }
            })

            seeds = [...nonmappedseeds]
        })

        seeds = [...mappedseeds, ...nonmappedseeds]
    })
 
    console.log("final seeds", seeds)

    let ans = seeds[0].start
    seeds.forEach((s) => {
        if (s.start < ans) {
            ans = s.start
        }
    })

    console.log(ans)
});
