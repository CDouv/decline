import { useState } from "react";
import Unknown from "./inputs/Unknown";
import Known from "./inputs/Known";

export const Parameter = ({
  parameter,
  onToggle,
  changeInput,
  clearInput,
  segmentNumber,
  toggleUnits,
}) => {
  //function to render input
  const renderInput = () => {
    if (parameter.calculate) {
      return (
        <Known
          parameter={parameter}
          changeInput={changeInput}
          segmentNumber={segmentNumber}
        />
      );
    } else {
      return (
        <Unknown
          parameter={parameter}
          changeInput={changeInput}
          clearInput={clearInput}
        />
      );
    }
  };

  return (
    <div className="parameter">
      <input
        type="checkbox"
        onClick={() => {
          onToggle(parameter.symbol, segmentNumber);
        }}
      />

      <div className="symbol">{parameter.symbol}</div>

      {renderInput()}

      <div
        className="units"
        onClick={() => {
          toggleUnits(parameter.symbol, segmentNumber);
        }}
      >
        {parameter.units}
      </div>
    </div>
  );
};

export default Parameter;
