export function formatIsoDate(isoDateStr: string): string {
  const isoDate = new Date(isoDateStr);
  const now = new Date();

  // Function to format the time in a 12-hour format with AM/PM
  const formatTime = (date: Date) => {
    let hours = date.getHours();
    const minutes = date.getMinutes();
    const amPm = hours >= 12 ? "PM" : "AM";
    hours = hours % 12;
    hours = hours ? hours : 12; // Adjust hour 0 to 12 for AM/PM
    return `${hours}:${minutes.toString().padStart(2, "0")} ${amPm}`;
  };

  // Get dates without the time component for comparison
  const isoDateOnly = new Date(isoDate.getFullYear(), isoDate.getMonth(), isoDate.getDate());
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);

  if (isoDateOnly.getTime() === today.getTime()) {
    return `Today at ${formatTime(isoDate)}`;
  } else if (isoDateOnly.getTime() === yesterday.getTime()) {
    return `Yesterday at ${formatTime(isoDate)}`;
  } else {
    // Format the date in "Month day, year" format
    const monthNames = [
      "January",
      "February",
      "March",
      "April",
      "May",
      "June",
      "July",
      "August",
      "September",
      "October",
      "November",
      "December",
    ];
    const formattedDate = `${monthNames[isoDate.getMonth()]} ${isoDate.getDate()}, ${isoDate.getFullYear()}`;
    return `${formattedDate} at ${formatTime(isoDate)}`;
  }
}
