const BASE = 'http://127.0.0.1:9999'

export async function fetchSysinfo() {
  const res = await fetch(`${BASE}/api/sysinfo`, { cache: 'no-store' })
  if (!res.ok) throw new Error(`Daemon returned ${res.status}`)
  return res.json()
}

export async function fetchProjects() {
  const res = await fetch(`${BASE}/api/projects`, { cache: 'no-store' })
  if (!res.ok) throw new Error(`Projects API returned ${res.status}`)
  return res.json()
}

export async function addEntry(projectId, text, isTodo) {
  await fetch(`${BASE}/api/projects/${projectId}/add`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ text, is_todo: isTodo }),
  })
}

export async function toggleEntry(projectId, line) {
  await fetch(`${BASE}/api/projects/${projectId}/toggle/${line}`, { method: 'POST' })
}

export async function updateEntry(projectId, line, text) {
  await fetch(`${BASE}/api/projects/${projectId}/update/${line}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ text }),
  })
}

export async function deleteEntry(projectId, line) {
  await fetch(`${BASE}/api/projects/${projectId}/delete/${line}`, { method: 'DELETE' })
}
