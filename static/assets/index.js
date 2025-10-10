const deptroot = document.getElementById('dept-root');

function formatDepartment(depts){
  return depts.map(e => `${e.deptid}\t${e.name}`).join('\n');
}

fetch('/dept').then(e=>e.json()).then(e=>{
  deptroot.innerText = formatDepartment(e);
}).catch(err=>{
  deptroot.innerText = 'Failed!';
  console.error(err);
})
