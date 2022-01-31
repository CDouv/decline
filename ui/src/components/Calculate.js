import React from "react";

export const Calculate = ({ countUnknowns, exportParameters, sendJSON }) => {
  return (
    <div>
      <button className="button" onClick={() => sendJSON()}>
        Calculate
      </button>
    </div>
  );
};

export default Calculate;
