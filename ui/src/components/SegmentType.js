import React from "react";

export const SegmentType = ({ changeSegmentType, segmentNumber }) => {
  // need segment number and segments

  return (
    <div>
      Decline Type
      <select
        className="select"
        onChange={(e) => {
          const selectedSegmentType = e.target.value;

          changeSegmentType(segmentNumber, selectedSegmentType);
        }}
      >
        <option value="exponential">Exponential</option>
        <option value="hyperbolic">Hyperbolic</option>
      </select>
    </div>
  );
};

export default SegmentType;
