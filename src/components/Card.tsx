import { invoke } from "@tauri-apps/api/core";
import ClockRegular from "../assets/icons/ClockRegular.svg"
export default function Card({heading, duedate, id, setDescription }: {heading: string, duedate: string, id: string, setDescription: Function}) {

    function formatDueDate(rawDueDate: string): string {
        try {
            // Extract the date and time using regex
            const dateTimeRegex = />([^<]+)<\/a>,\s*([^<]+)/;
            const match = rawDueDate.match(dateTimeRegex);

            if (match && match.length >= 3) {
                const date = match[1].trim(); // e.g., "Tuesday, 25 March"
                const time = match[2].trim(); // e.g., "12:00 PM"
                return `${date}, ${time}`;
            }

            // If we can't parse it, return the original but clean up HTML entities
            return rawDueDate
                .replace(/&amp;/g, '&')
                .replace(/<[^>]*>/g, '')
                .trim();
        } catch (error) {
            // Fallback in case of any error
            return "Date not available";
        }
    }
    function extractCourseCode(courseName: string): string {
        try {
            // Extract text within brackets using regex
            const courseCodeRegex = /\(([^)]+)\)/;
            const match = courseName.match(courseCodeRegex);

            if (match && match.length >= 2) {
                return match[1]; // Return just the text within brackets
            }

            // If no match found, return the original course name
            return courseName;
        } catch (error) {
            // Fallback in case of any error
            return courseName;
        }
    }
    function displayDueDate(rawDueDate: string): string {
        try {
            // Split the date string by commas
            const parts = rawDueDate.split(', ');

            if (parts.length >= 2) {
                // Get the date part (e.g., "25 March")
                const datePart = parts[1].trim(); // This is the "25 March" part

                // Get the time part (e.g., "12:00 PM")
                const timePart = parts.length >= 3 ? parts[2].trim() : "";

                return `${datePart}, ${timePart}`;
            }

            // If we can't parse it correctly, return the original date
            return rawDueDate;
        } catch (error) {
            // Fallback in case of any error
            return rawDueDate;
        }
    }

    function handleOpenAssignment(id: string) {
        return async () => {
            let assignmentDetails = JSON.parse(await invoke("open_assignment_lms", { id }));
            console.log(assignmentDetails);
            setDescription(assignmentDetails);
        }
    }

    return (
        <div className="flex items-center justify-center bg-secondary-500 h-20 rounded-3xl p-4 mt-5 flex-col font-primary hover:bg-secondary-600 hover:cursor-pointer" onClick={handleOpenAssignment(id.toString())}>
            <p className="font-bold">{extractCourseCode(heading)}</p>
            <div className="flex items-center gap-2">
                <img className="h-5 object-contain " src={ClockRegular} alt="Clock regular" />
                <p className="text-xs ">{displayDueDate(formatDueDate(duedate))}</p>
            </div>
        </div>
    )
}