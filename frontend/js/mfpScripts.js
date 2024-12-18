document.addEventListener('DOMContentLoaded', () => {
  const addBtn = document.getElementById('addBtn');
  addBtn.addEventListener('click', addboxes);

  // Log Workout button
  const logBtn = document.getElementById('submit');
  logBtn.addEventListener('click', () => CreatePlan());
});

// Add new dynamic fields
function addboxes() {
  const originalContainer = document.querySelector('.dynitems');
  if (!originalContainer) {
    console.error("Original container not found.");
    return;
  }

  const wrapper = document.createElement('div');
  wrapper.classList.add('dynitems-wrapper');

  wrapper.innerHTML = `
      <input type="text" name="WKItem" placeholder="Add Exercise" class="input-field">
      <select name="WKtag" class="input-select">
          <option value="null">Please Select</option>
          <option value="Chest">Chest</option>
          <option value="Shoulders">Shoulders</option>
          <option value="Back">Back</option>
          <option value="Arms">Arms</option>
          <option value="Legs">Legs</option>
          <option value="Core">Core</option>
      </select>
      <input type="number" name="Areps" placeholder="Reps" min="0" max="99" class="input-field">
      <input type="number" name="Asets" placeholder="Sets" min="0" max="99" class="input-field">
      <button type="button" class="btn-del-input">Delete</button>
  `;

  // Attach delete event
  wrapper.querySelector('.btn-del-input').addEventListener('click', (e) => {
    wrapper.remove();
  });

  originalContainer.appendChild(wrapper);
}

// Gather and log the plan data
function CreatePlan() {
  const wrappers = document.querySelectorAll('.dynitems-wrapper');
  const wkplan = [];

  const name = document.getElementById("workoutName").value.trim();
  const comment = document.getElementById("workoutComment").value.trim();



  console.log(wrappers);


  wrappers.forEach((wrapper) => {
    console.log(wrapper.children[1].value);

    const exercise = wrapper.children[0].value || '';
    const tag = wrapper.children[1].value || '';
    const reps = parseInt(wrapper.children[2].value) || 0;
    const sets = parseInt(wrapper.children[3].value) || 0;

    if (exercise && tag !== 'null' && reps && sets) {
      wkplan.push({ name, tag, reps, sets });
    }
  });

  console.log("Workout Plan:", wkplan);

  let workout = {
    name, comment, exercises: wkplan
  }

  console.log(workout);


  fetch('/api/workout/upload', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(workout),
  }).then((res) => {
    console.log(res);
    alert("Uploaded workout successfully");
  }).catch((e) => {
    console.log(e);

  })

}

// Sidebar toggle
function toggleSidebar() {
  const sidebar = document.getElementById("sidebar");
  sidebar.classList.toggle("expanded");
}

// Logout functionality
function handleLogout() {
  if (confirm("Are you sure you want to log out?")) {
    document.cookie = "sessionToken=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
    localStorage.clear();
    window.location.href = "Login.HTML";
  }
}
