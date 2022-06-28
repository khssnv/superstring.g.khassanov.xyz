pub mod naive {
    use itertools::Itertools;

    pub fn shortest_superstring(substrings: Vec<String>) -> String {
        let n = substrings.len();
        let mut cost = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                let pair = (substrings[i].clone(), substrings[j].clone());
                let max_overlap = pair.0.len().min(pair.1.len());
                for k in (1..max_overlap).rev() {
                    // println!(
                    //     "k={:?}, pair.0[..k]={:?}, pair.1[(pair.1.len() - k)..]={:?}",
                    //     k,
                    //     &pair.0[..k],
                    //     &pair.1[(pair.1.len() - k)..]
                    // );
                    if pair.0[pair.0.len() - k..] == pair.1[..k] {
                        cost[i][j] = k;
                        break;
                    }
                }
            }
        }

        let mut dp = vec![vec![(500, String::from("")); n]; 1 << n];

        for i in 0..n {
            dp[1 << i][i] = (substrings[i].len(), substrings[i].clone());
        }
        // println!("cost={:?}", cost);

        for bitmask in 0..(1 << n) {
            let mut bits: Vec<usize> = vec![];
            for j in 0..n {
                if (bitmask & (1 << j)) != 0 {
                    bits.push(j);
                }
            }
            // println!("bits={:?}", bits);
            for idxs in bits.into_iter().permutations(2) {
                // println!("idxs={:?}", idxs);
                let trg = idxs[0];
                let src = idxs[1];
                let candidate = format!(
                    "{}{}",
                    dp[bitmask ^ (1 << trg)][src].1.clone(),
                    &substrings[trg][cost[src][trg]..],
                );
                // println!(
                //     "trg={:?}, src={:?}, candidate={:?}, prefix={:?}, suffix={:?}, key={:?}, cost[src][trg]={:?}",
                //     trg,
                //     src,
                //     candidate,
                //     dp[bitmask ^ (1 << trg)][src].1.clone(),
                //     &substrings[trg][cost[src][trg]..],
                //     bitmask ^ (1 << trg),
                //     cost[src][trg],
                // );
                // dp[bitmask][trg] = (candidate.len(), candidate);

                if dp[bitmask][trg].0 > candidate.len() {
                    dp[bitmask][trg] = (candidate.len(), candidate);
                }
            }
        }

        dp[dp.len() - 1]
            .iter()
            .min_by_key(|item| item.0)
            .unwrap()
            .1
            .clone()
    }
}

pub mod tsp {
    fn tsp(
        dp: &mut Vec<Vec<Option<(usize, Option<usize>)>>>,
        graph: &Vec<Vec<usize>>,
        set: usize,
        dest: usize,
    ) {
        if dp[set][dest].is_some() {
            return;
        }
    
        let prev_set = set & (!(1 << dest));
        let mut best = usize::MAX;
        let mut best_i = 0;
    
        for i in 0..graph.len() {
            if i == dest {
                continue;
            };
    
            if ((1 << i) & prev_set) == 0 {
                continue;
            }
    
            tsp(dp, graph, prev_set, i);
    
            let w = (dp[prev_set][i].as_ref().unwrap().0) + graph[i][dest];
    
            if w < best {
                best = w;
                best_i = i;
            }
        }
        dp[set][dest] = Some((best, Some(best_i)));
    }
    
    pub fn shortest_superstring(substrings: Vec<String>) -> String {
        let substrings_count = substrings.len();
        // println!("{:?}", substrings_count);
    
        let mut graph = vec![vec![0; substrings_count]; substrings_count];
    
        for i in 0..substrings_count {
            for j in 0..substrings_count {
                if i == j {
                    continue;
                }
    
                let mut overlap = 0;
                let pair = (substrings[i].clone(), substrings[j].clone());
                let max_overlap = pair.0.len().min(pair.1.len());
    
                for k in 1..=max_overlap {
                    if pair.0[..k] == pair.1[(pair.1.len() - k)..] {
                        overlap = k;
                    }
                }
    
                graph[j][i] = pair.0.len() - overlap;
            }
        }
    
        let mut dp = vec![vec![None; substrings_count]; 1 << substrings_count];
        let bitmask_len = (2u32.pow((substrings_count) as u32) - 1) as usize;
        // println!("bitmask_len: {:?}", bitmask_len);
        // println!("dp: {:?}", dp);
    
        for i in 0..substrings_count {
            let set = 1 << i;
            dp[set][i] = Some((substrings[i].len(), None));
        }
        // println!("dp({:?}): {:?}", dp.len(), dp);
    
        let mut best = (usize::MAX, None);
    
        for i in 0..substrings_count {
            tsp(&mut dp, &graph, bitmask_len, i);
            let s = (dp[bitmask_len][i].as_ref().unwrap()).0;
            if best.0 > s {
                best = (s, Some(i));
            }
        }
        // println!("dp: {:?}", dp);
    
        let mut order = vec![];
        // println!("best: {:?}", best);
        let mut cur = Some((best.1.unwrap(), bitmask_len));
        // println!("cur: {:?}", cur);
        while let Some((index, set)) = cur {
            order.push(index);
            let next_set = set & (!(1 << index));
            if let Some(&(_, o_n)) = dp[set][index].as_ref() {
                if let Some(n) = o_n {
                    cur = Some((n, next_set));
                } else {
                    cur = None;
                }
            } else {
                cur = None;
            }
            // println!("order: {:?}", order);
        }
        // println!("cur: {:?}", cur);
        order.reverse();
    
        let mut answer = substrings[order[0]].clone();
        for i in 1..(order.len()) {
            let add = graph[order[i - 1]][order[i]];
            let size = substrings[order[i]].len();
            answer.push_str(&substrings[order[i]][size - add..]);
        }
    
        answer
    }
}
