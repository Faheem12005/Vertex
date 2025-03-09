import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";

interface Assignments {
  name: string,
  description: string,
  instanceId: string,
  courseName: string,
  dueDate: string,
}

const fetchAssignments = async () => {
  try {
    let response = JSON.parse(await invoke("fetch_assignments"));
    console.log(response)
    return response[0].data.events.map((event: any) => ({
      name: event.name,
      description: event.description,
      instanceId: event.instance,
      courseName: event.course.fullname, 
      dueDate: event.formattedtime, 
    }));
  } catch (error) {
    console.error("Failed to fetch assignments:", error);
    return null;
  }
};

const renderAssignment = (assignment: Assignments) => {
  return (
    <div key={assignment.instanceId} className="p-4 border rounded shadow-md">
      <h2 className="text-xl font-bold">{assignment.name}</h2>
      <p>{assignment.description}</p>
      <p><strong>Course:</strong> {assignment.courseName}</p>
      <p><strong>Due Date:</strong> {assignment.dueDate}</p>
    </div>
  );
};


export default function Dashboard() {
  const [assignments, setAssignments] = useState<null | Assignments[]>(null);

  useEffect(() => {
    fetchAssignments().then(setAssignments);
  }, []);

  return (
    <div className="h-screen w-screen flex justify-center flex-col gap-8 items-center bg-white">
      <p>Welcome! {localStorage.getItem("username")}</p>
      <div>{assignments?.map(renderAssignment)}</div>
    </div>
  );
}
