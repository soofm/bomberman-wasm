use crate::models::Tiles;
use std::collections::HashMap;

pub fn find_available_paths(position: (i32, i32), tiles: &Tiles) -> HashMap<(i32, i32), (i32, i32)> {    
  let mut visited = tiles.map_blocked();
  let mut queue = Vec::new();
  let mut result: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
  let row_idx = tiles.width as usize;

  queue.push(position);
  while !queue.is_empty() {
    let (col, row) = queue.remove(0);

    let index = (row * tiles.width + col) as usize;
    visited[index] = true;
    if col > 0 && !visited[index - 1] {
      queue.push((col - 1, row));
      result.insert((col - 1, row), (col, row));
    }
    if col < tiles.width - 1 && !visited[index + 1] {
      queue.push((col + 1, row));
      result.insert((col + 1, row), (col, row));
    }
    if row > 0 && !visited[index - row_idx] {
      queue.push((col, row - 1));
      result.insert((col, row - 1), (col, row));
    }
    if row < tiles.height - 1 && !visited[index + row_idx] {
      queue.push((col, row + 1));
      result.insert((col, row + 1), (col, row));
    }
  }
  result
}

pub fn rebuild_path(target: (i32, i32), available_paths: HashMap<(i32, i32), (i32, i32)>) -> Vec<(i32, i32)> {
  let mut total_path = vec![target];
  let mut current = target;
  while let Some(position) = available_paths.get(&current) {
    current = *position;
    total_path.push(current);
  }
  total_path
}