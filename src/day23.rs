use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn find_three_peers(connections: &HashMap<&str, Vec<&str>>, name: &str) -> Vec<String> {
    let peers = connections.get(name).unwrap();
    let mut networks: Vec<Vec<&str>> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();
    let mut result: Vec<String> = Vec::new();

    for p_name in peers {
        if *p_name != name {
            if !seen.contains(p_name) {
                seen.insert(p_name);
                let mut lan: Vec<&str> = Vec::new();
                lan.push(p_name);
                let peers_of_peer = connections.get(p_name).unwrap();
                for peer_of_peer in peers_of_peer {
                    if peers.contains(peer_of_peer) {
                        seen.insert(peer_of_peer);
                        lan.push(peer_of_peer);
                    }
                }
                networks.push(lan);
            }
        }
    }

    for net in networks {
        for i in 0..(net.len() - 1) {
            for j in (i + 1)..net.len() {
                let mut chars: Vec<char> =
                    format!("{}{}{}", name, net[i], net[j]).chars().collect();
                chars.sort();
                let net_name = chars.iter().collect::<String>();
                result.push(net_name);
            }
        }
    }

    result
}

fn is_connected_to_all(
    name: &str,
    connections: &HashMap<&str, Vec<&str>>,
    network: &Vec<String>,
) -> bool {
    network.iter().all(|comp| {
        let connected_computers = connections.get(comp.as_str()).unwrap();

        connected_computers.contains(&name)
    })
}

fn find_largest(
    name: &str,
    connections: &HashMap<&str, Vec<&str>>,
    network: Vec<String>,
    all_networks: &mut HashSet<String>,
) {
    let mut net = network.clone();
    net.sort();
    let key = net.join(",");
    if all_networks.contains(&key) {
        return;
    }

    all_networks.insert(key);
    let peers = connections.get(name).unwrap();

    for peer in peers {
        if !network.contains(&peer.to_string()) {
            if is_connected_to_all(*peer, connections, &network) {
                let mut extedned_net = network.clone();
                extedned_net.push(peer.to_string());
                find_largest(peer, connections, extedned_net, all_networks);
            }
        }
    }
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

    contents.lines().for_each(|l| {
        let names: Vec<&str> = l.split('-').collect();
        connections
            .entry(names[0])
            .or_insert(Vec::new())
            .push(names[1]);
        connections
            .entry(names[1])
            .or_insert(Vec::new())
            .push(names[0]);
    });

    let mut all_nets: HashSet<String> = HashSet::new();

    for computer_name in connections.keys() {
        if (*computer_name).starts_with('t') {
            let peers = find_three_peers(&connections, computer_name);
            for peer in peers {
                all_nets.insert(peer);
            }
        }
    }

    let answer = all_nets.len();
    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

    contents.lines().for_each(|l| {
        let names: Vec<&str> = l.split('-').collect();
        connections
            .entry(names[0])
            .or_insert(Vec::new())
            .push(names[1]);
        connections
            .entry(names[1])
            .or_insert(Vec::new())
            .push(names[0]);
    });

    let mut all_nets: HashSet<String> = HashSet::new();

    for computer_name in connections.keys() {
        find_largest(
            computer_name,
            &connections,
            vec![computer_name.to_string()],
            &mut all_nets,
        )
    }

    let answer = all_nets
        .iter()
        .max_by(|a, b| (a.len()).cmp(&b.len()))
        .unwrap();
    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
