use crate::{SwarmMetric, SwarmPos, SWARM_SIZE};

pub fn metric_dist(
    sim_swarm_pos: &Vec<SwarmPos>,
    real_swarm_pos: &Vec<SwarmPos>,
    swarm_mode_dist: f64,
    density_radius: f64,
) -> SwarmMetric {
    let sim_swarm_metic = to_metic(sim_swarm_pos, swarm_mode_dist, density_radius);
    let real_swarm_metic = to_metic(real_swarm_pos, swarm_mode_dist, density_radius);
    let mut sum = SwarmMetric::default();

    for (sim, real) in sim_swarm_metic.iter().zip(real_swarm_metic.iter()) {
        for i in 0..sum.len() {
            sum[i] += (sim[i] - real[i]).powi(2);
        }
    }

    for i in 0..sum.len() {
        sum[i] = sum[i].sqrt();
    }
    return sum;
}

/// apples the swarm_metic function to all positions
pub fn to_metic(
    all_swarm_pos: &Vec<SwarmPos>,
    swarm_mode_dist: f64,
    density_radius: f64,
) -> Vec<SwarmMetric> {
    let mut result = Vec::with_capacity(all_swarm_pos.len() - 1);

    let mut all_swarm_pos_it = all_swarm_pos.iter();
    let swarm_origen = all_swarm_pos_it.next().unwrap();
    let mut pre_swarm_pos = swarm_origen;

    for swarm_pos in all_swarm_pos_it {
        result.push(swarm_metic(
            swarm_pos,
            &swarm_origen,
            &pre_swarm_pos,
            swarm_mode_dist,
            density_radius,
        ));
        pre_swarm_pos = swarm_pos;
    }

    return result;
}

/// calculates the 8 swarm mercies for a specific swarm position
///
/// 9 values are returned because the first value is the center_of_mass x, and the second is the center_of_mass y.
pub fn swarm_metic(
    swarm_pos: &SwarmPos,
    swarm_origen: &SwarmPos,
    pre_swarm_pos: &SwarmPos,
    swarm_mode_dist: f64,
    density_radius: f64,
) -> SwarmMetric {
    let center_of_mass = center_of_mass(swarm_pos);
    let max_swarm_shift = max_dist(swarm_pos, pre_swarm_pos);
    let swarm_mode_index = swarm_mode_index(swarm_pos, &center_of_mass, swarm_mode_dist);
    let longest_path = max_dist(swarm_pos, swarm_origen);
    let max_radius = max_dist(swarm_pos, &[center_of_mass; SWARM_SIZE]);
    let local_density = local_density(swarm_pos, density_radius);
    let nears_neighbor_distance = nears_neighbor_distance(swarm_pos);
    let beta_index = beta_index(swarm_pos);

    return [
        center_of_mass.0,
        center_of_mass.1,
        max_swarm_shift,
        swarm_mode_index,
        longest_path,
        max_radius,
        local_density,
        nears_neighbor_distance,
        beta_index,
    ];
}

pub fn center_of_mass(swarm_pos: &SwarmPos) -> (f64, f64) {
    let mut sum = (0.0, 0.0);
    for pos in swarm_pos {
        sum.0 += pos.0;
        sum.1 += pos.1;
    }

    sum.0 = sum.0 / SWARM_SIZE as f64;
    sum.1 = sum.1 / SWARM_SIZE as f64;

    return sum;
}

/// 7-Average nearest neighbour distance is the sum of the distance to the nearest neighbour of each agent averaged over the total number of agents.
pub fn nears_neighbor_distance(swarm_pos: &SwarmPos) -> f64 {
    let mut sum = 0.0;
    for self_pos in swarm_pos {
        let mut nn_dist = 0.0;
        for other_pos in swarm_pos {
            if other_pos != self_pos && dist(self_pos, other_pos) > nn_dist {
                nn_dist = dist(self_pos, other_pos)
            }
        }
        sum += nn_dist
    }
    return sum / SWARM_SIZE as f64;
}

/// 6-Average local density is the sum of the number of agents in the local radius r of each agent averaged over the total number of agents.
pub fn local_density(swarm_pos: &SwarmPos, radius: f64) -> f64 {
    let mut sum = 0;
    for self_pos in swarm_pos {
        let mut num_of_neighbor = 0;
        for other_pos in swarm_pos {
            if other_pos != self_pos && dist(self_pos, other_pos) < radius {
                num_of_neighbor += 1;
            }
            sum += num_of_neighbor;
        }
    }
    return sum as f64 / SWARM_SIZE as f64;
}

/// calculates the distance of all the points of swarm_pos1 and swarm_pos2 and returns the maximum
pub fn max_dist(swarm_pos1: &SwarmPos, swarm_pos2: &SwarmPos) -> f64 {
    let mut max_dist = 0.0;

    for (pos1, pos2) in swarm_pos1.iter().zip(swarm_pos2.iter()) {
        if dist(pos1, pos2) > max_dist {
            max_dist = dist(pos1, pos2);
        }
    }
    return max_dist;
}

/// 3-Swarm mode index is used to measure the frequency of the swarm motion.
///
/// It is computed as the distance between the center of mass and the swarm mode at each time-step t.
///
/// The swarm mode is defined as a location in the x and the y direction with maximum frequency among all agent's locations.
///
/// The frequency of location l in the x or the y direction is computed using the following formula:
///
///                 n
/// frequency(l) = sum 1
///                i=0
///             distance(l, li) < 0.1
///
pub fn swarm_mode_index(
    swarm_pos: &SwarmPos,
    center_of_mass: &(f64, f64),
    swarm_mode_dist: f64,
) -> f64 {
    let mut swarm_mode: Vec<(usize, usize)> = Vec::with_capacity(SWARM_SIZE);
    for pos in swarm_pos {
        let mut neighbor_count = (0, 0);
        for other_pos in swarm_pos {
            if pos == other_pos {
                continue;
            }
            if f64::abs(pos.0 - other_pos.0) < swarm_mode_dist {
                neighbor_count.0 += 1;
            }
            if f64::abs(pos.1 - other_pos.1) < swarm_mode_dist {
                neighbor_count.1 += 1;
            }
        }
        swarm_mode.push(neighbor_count);
    }

    let mut max_mode = (0, 0);
    let mut index = (0, 0);

    for (i, mode) in swarm_mode.iter().enumerate() {
        if mode.0 > max_mode.0 {
            max_mode.0 = mode.0;
            index.0 = i;
        }
        if mode.1 > max_mode.1 {
            max_mode.1 = mode.1;
            index.1 = i;
        }
    }

    let swarm_mode_x = swarm_pos[index.0].0;
    let swarm_mode_y = swarm_pos[index.1].1;
    let swarm_mode_index = dist(center_of_mass, &(swarm_mode_x, swarm_mode_y));
    return swarm_mode_index;
}

pub fn dist(vec1: &(f64, f64), vec2: &(f64, f64)) -> f64 {
    return f64::sqrt((vec1.0 - vec2.0) * (vec1.0 - vec2.0))
        + ((vec1.1 - vec2.1) * (vec1.1 - vec2.1));
}

/// The beta index is a metric that measures
/// the connectivity of the graph by dividing the number of paths between nodes
/// by the number of nodes in the graph. For the swarm beta index, the path is
/// assumed to be connecting two agents if the distance between them is less than
/// the average distance. Average distance is computed as the sum of the distances
/// among all the agents over the total number of agents
pub fn beta_index(swarm_pos: &SwarmPos) -> f64 {
    let mut total_distance = 0.0;

    for i in 0..SWARM_SIZE {
        for j in (i + 1)..SWARM_SIZE {
            let dx = swarm_pos[i].0 - swarm_pos[j].0;
            let dy = swarm_pos[i].1 - swarm_pos[j].1;
            let distance = (dx.powi(2) + dy.powi(2)).sqrt();
            total_distance += distance;
        }
    }

    let average_dist = total_distance / (SWARM_SIZE * (SWARM_SIZE - 1) / 2) as f64;

    let mut paths_count = 0;
    for i in 0..SWARM_SIZE {
        for j in (i + 1)..SWARM_SIZE {
            let dx = swarm_pos[i].0 - swarm_pos[j].0;
            let dy = swarm_pos[i].1 - swarm_pos[j].1;
            let distance = (dx.powi(2) + dy.powi(2)).sqrt();

            if distance < average_dist {
                paths_count += 1;
            }
        }
    }
    return paths_count as f64 / SWARM_SIZE as f64;
}
