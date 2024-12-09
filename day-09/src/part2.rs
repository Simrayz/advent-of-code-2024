#[derive(Debug)]
struct Chunk {
    file_id: usize,
    count: usize,
    index: usize,
}

pub fn process(input: &str) -> miette::Result<String> {
    let chunks = get_reversed_chunks(input);
    let moved_chunks = get_moved_chunks(input, &chunks);
    let total_sum = get_check_sum(input, &moved_chunks);

    Ok(total_sum.to_string())
}

fn get_reversed_chunks(input: &str) -> Vec<Chunk> {
    let expanded_max_index: usize = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum();

    let idx_char_tuples = (0..input.len()).rev().zip(input.chars().rev());
    let mut current_index = expanded_max_index;
    idx_char_tuples
        .filter_map(|(idx, c)| {
            let indices_count = c.to_digit(10).unwrap() as usize;
            current_index -= indices_count;

            if idx % 2 == 0 {
                Some(Chunk {
                    index: current_index,
                    count: indices_count,
                    file_id: idx / 2,
                })
            } else {
                None
            }
        })
        .collect()
}

fn get_moved_chunks(input: &str, chunks: &Vec<Chunk>) -> Vec<Chunk> {
    let mut empty_chunks = get_empty_chunks(input);
    let mut moved_chunks: Vec<Chunk> = vec![];
    for chunk in chunks {
        let Some(empty) = empty_chunks
            .iter_mut()
            .find(|(idx, space)| chunk.count <= *space && *idx < chunk.index)
        else {
            continue;
        };

        moved_chunks.push(Chunk {
            index: empty.0,
            ..*chunk
        });
        empty.0 += chunk.count;
        empty.1 -= chunk.count;
    }

    moved_chunks
}

fn get_empty_chunks(input: &str) -> Vec<(usize, usize)> {
    let (_, empty_chunks) = input.chars().enumerate().fold(
        (0, vec![]),
        |(mut expanded_idx, mut empty_chunks), (idx, c)| {
            let indices_count = c.to_digit(10).unwrap() as usize;
            if idx % 2 != 0 {
                empty_chunks.push((expanded_idx, indices_count))
            }
            expanded_idx += indices_count;
            (expanded_idx, empty_chunks)
        },
    );

    empty_chunks
}

fn get_check_sum(input: &str, moved_chunks: &Vec<Chunk>) -> usize {
    // Sum files that weren't moved
    let mut base_index = 0;
    let mut static_sum = 0;
    for (idx, char) in input.chars().enumerate() {
        let indices_count = char.to_digit(10).unwrap() as usize;
        let file_id = idx / 2;

        for expanded_idx in base_index..(base_index + indices_count) {
            if idx % 2 == 0 && !moved_chunks.iter().any(|c| c.file_id == file_id) {
                static_sum += expanded_idx * file_id;
            }
        }

        base_index += indices_count;
    }

    // Sum files that were moved
    let mut moved_sum = 0;
    for chunk in moved_chunks {
        for idx in chunk.index..(chunk.index + chunk.count) {
            moved_sum += idx * chunk.file_id;
        }
    }

    return static_sum + moved_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../input1.txt"));
        assert_eq!("2858", result?);
        Ok(())
    }
}
