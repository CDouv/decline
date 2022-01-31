import React from "react";

export const DeleteSegment = ({ deleteSegment }) => {
  return (
    <div>
      <button className="button" onClick={() => deleteSegment()}>
        Delete Segment
      </button>
    </div>
  );
};

export default DeleteSegment;
