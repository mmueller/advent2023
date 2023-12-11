use crate::advent::AdventSolver;
use crate::util::io;
use anyhow::{format_err, Error};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;
use std::ops::Range;
use strum::{self, EnumString};

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self, input_path: &str) -> Result<(), Error> {
        let input = io::read_file_as_lines(input_path)?;
        let almanac = Almanac::new(&input)?;
        println!("Read almanac.");
        println!(
            "Lowest location number: {}",
            almanac
                .seeds_to_plant
                .iter()
                .map(|&s| almanac.location_for_seed(s))
                .min()
                .unwrap()
        );

        let mut lowest_location = u64::MAX;
        for (&seed_start, &length) in almanac.seeds_to_plant.iter().tuples() {
            let mut seed = seed_start;
            while seed < seed_start + length {
                let (location, step) = almanac.optimized_location_for_seed(seed);
                if location < lowest_location {
                    lowest_location = location;
                }
                seed += step;
            }
        }
        println!(
            "Considering seed ranges, lowest location number: {}",
            lowest_location
        );
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, EnumString, Eq, Hash, PartialEq)]
#[strum(serialize_all = "lowercase")]
enum GardenResource {
    Fertilizer,
    Humidity,
    Light,
    Location,
    Seed,
    Soil,
    Temperature,
    Water,
}

lazy_static! {
    static ref SEEDS_RE: Regex = Regex::new(r"^seeds: (?P<seeds>[0-9 ]+)$").unwrap();
    static ref MAP_START_RE: Regex =
        Regex::new(r"^(?P<source>[a-z]+)-to-(?P<dest>[a-z]+) map:").unwrap();
    static ref MAP_ROW_RE: Regex =
        Regex::new(r"^(?P<dest>\d+) (?P<source>\d+) (?P<len>\d+)$").unwrap();
    static ref CONVERSIONS: Vec<GardenResource> = vec![
        GardenResource::Seed,
        GardenResource::Soil,
        GardenResource::Fertilizer,
        GardenResource::Water,
        GardenResource::Light,
        GardenResource::Temperature,
        GardenResource::Humidity,
        GardenResource::Location,
    ];
}

struct Almanac {
    seeds_to_plant: Vec<u64>,
    maps: HashMap<(GardenResource, GardenResource), Vec<(Range<u64>, u64)>>,
}

impl Almanac {
    fn new<S: AsRef<str>>(input: &Vec<S>) -> Result<Almanac, Error> {
        // Almanac fields
        let mut seeds_to_plant = Vec::new();
        let mut maps: HashMap<(GardenResource, GardenResource), Vec<(Range<u64>, u64)>> =
            HashMap::new();
        // When we're reading resource maps (e.g. "seed-to-soil") these will be populated.
        let mut current_source: Option<GardenResource> = None;
        let mut current_dest: Option<GardenResource> = None;

        for line in input.iter() {
            let line = line.as_ref();
            if line.len() == 0 {
                // Blank line signals a change to a new map
                current_source = None;
                current_dest = None;
            } else if let Some(caps) = SEEDS_RE.captures(line) {
                // The "seeds to plant" line.
                seeds_to_plant = caps["seeds"]
                    .split_whitespace()
                    .map(|s| s.parse::<u64>())
                    .collect::<Result<Vec<_>, _>>()?;
            } else if let Some(caps) = MAP_START_RE.captures(line) {
                // A new source-to-dest map is starting
                current_source = Some(caps["source"].try_into()?);
                current_dest = Some(caps["dest"].try_into()?);
            } else if let Some(caps) = MAP_ROW_RE.captures(line) {
                // Read a single line of a source-to-dest map
                let source = current_source.ok_or(format_err!("no map header: {}", line))?;
                let dest = current_dest.ok_or(format_err!("no map header: {}", line))?;
                let s_start = caps["source"].parse::<u64>()?;
                let d_start = caps["dest"].parse::<u64>()?;
                let len = caps["len"].parse::<u64>()?;
                maps.entry((source, dest))
                    .or_default()
                    .push((s_start..s_start + len, d_start));
            } else {
                return Err(format_err!("Unexpected line in input: {}", line));
            }
        }
        Ok(Almanac {
            seeds_to_plant,
            maps,
        })
    }

    fn convert_resource(&self, source: GardenResource, dest: GardenResource, value: u64) -> u64 {
        if let Some(ranges) = self.maps.get(&(source, dest)) {
            for (srange, dstart) in ranges.iter() {
                if srange.contains(&value) {
                    let offset = value - srange.start;
                    return dstart + offset;
                }
            }
        }
        value
    }

    // Implements the entire lookup chain described in part 1, assuming it is static.
    fn location_for_seed(&self, seed: u64) -> u64 {
        CONVERSIONS
            .iter()
            .tuple_windows()
            .fold(seed, |resource, (&source, &dest)| {
                self.convert_resource(source, dest, resource)
            })
    }

    // Returns the location and a suggested number of seeds to skip for the next attempt, based on
    // how far the next breakpoint is in the mappings. (The only points where the location could
    // possibly get lower while the seed number is increasing.)
    fn optimized_location_for_seed(&self, seed: u64) -> (u64, u64) {
        let mut step: u64 = u64::MAX;
        let mut value = seed;
        CONVERSIONS
            .iter()
            .tuple_windows()
            .for_each(|(&source, &dest)| {
                let range_maps = &self.maps[&(source, dest)];
                for (srange, dstart) in range_maps.iter() {
                    if srange.contains(&value) {
                        let offset = value - srange.start;
                        step = min(step, srange.end - value);
                        value = dstart + offset;
                        break;
                    }
                }
                // If we're not inside a mapping range (in the fallthrough 1:1 behavior), we need
                // to consider where the next mapping begins.
                for (srange, _dstart) in range_maps.iter() {
                    if srange.start > value {
                        step = min(step, srange.start - value);
                    }
                }
            });
        (value, step)
    }
}

#[cfg(test)]
mod tests {
    use super::Almanac;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref EX_IN: Vec<&'static str> = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];
    }

    #[test]
    fn test_lookup_seed_location() {
        let almanac = Almanac::new(&EX_IN).unwrap();
        assert_eq!(82, almanac.location_for_seed(79));
        assert_eq!(43, almanac.location_for_seed(14));
        assert_eq!(86, almanac.location_for_seed(55));
        assert_eq!(35, almanac.location_for_seed(13));
    }
}
