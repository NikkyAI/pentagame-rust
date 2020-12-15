function joinGame(_event) {
  let id = document.getElementById('game-id').value;
  location = `/games/join/${id}`;
}
