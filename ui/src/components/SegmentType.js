import React from "react";

export const SegmentType = ({ segmentNumber, segments, setSegments }) => {
  // need segment number and segments

  const changeSegmentType = (segmentNumber, selectedSegmentType) => {
    let segCopy = segments.map((seg) => {
      return { ...seg };
    });

    segCopy = segCopy.map((seg) => {
      if (seg.segmentNumber === segmentNumber) {
        let newParameters = seg.parameters.map((param) => {
          return { ...param, input: undefined };
        });
        let newSegment = {
          ...seg,
          forecastType: selectedSegmentType,
          parameters: newParameters,
        };

        return newSegment;
      }
    });
    return setSegments(segCopy);
  };

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
