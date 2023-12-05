#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
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
                // // Part 1
                // if (Number(s)) {
                //     seeds.push(Number(s))
                // }

                // Part 2
                if (si%2 === 1) {
                // if (si > 0) {
                    seeds.push({
                        start: Number(ss[si]),
                        // len: 1
                        len: Number(ss[si+1])
                    })
                }
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
        console.log("mapset", mapset)
        // console.log("seeds", seeds)
        let mappedseeds = []
        let nonmappedseeds = []
        mapset.forEach((mapping) => {
            console.log("mapping", mapping)
            console.log("seeds", seeds)
            nonmappedseeds = []
            seeds.forEach((s) => {
                // console.log("if", s.start+s.len, "<", mapping.start, "||", mapping.start+mapping.len, "<", s.start)
                if ((s.start+s.len-1 < mapping.start) || (mapping.start+mapping.len-1 < s.start)) {
                    nonmappedseeds.push(s)
                    console.log("skip", nonmappedseeds)
                    return // no overlap between seed and mapping
                }

                // Find the overlap
                let points = [s.start, s.start+s.len-1, mapping.start, mapping.start+mapping.len-1]                
                let sorted = points.sort((a, b) => a - b)
                let newlen = sorted[2] - sorted[1] + 1

                mappedseeds.push({
                    start: sorted[1] + mapping.delta,
                    len: newlen
                })
                console.log("mapped", mappedseeds)

                // TODO: get unmapped part and push it to nonmapped
                if (s.start === sorted[0] && mapping.start === sorted[1]) {
                    nonmappedseeds.push({
                        start: s.start,
                        len: sorted[1] - sorted[0] + 1
                    })
                    console.log("extra(1)", nonmappedseeds)
                    // console.log("unmapped(1)", unmappedseeds[unmappedseeds.length-1])
                } else if (sorted[2] === mapping.src+mapping.len-1 && sorted[3] === s.start+s.len-1) {
                    nonmappedseeds.push({
                        start: sorted[2],
                        len: sorted[3] - sorted[2] + 1
                    })
                    console.log("extra(2)", nonmappedseeds)
                    // console.log("unmapped(2)", unmappedseeds[unmappedseeds.length-1])
                }


            })

            seeds = [...nonmappedseeds]
        })

        seeds = [...mappedseeds, ...nonmappedseeds]
    })
 
    console.log("final seeds", seeds)

    let ans = seeds[0].start
    seeds.forEach((s) => {
        // For some reason 
        if (s.start < ans && s.start > 0) {
            ans = s.start
        }
    })

    console.log(ans)
});
