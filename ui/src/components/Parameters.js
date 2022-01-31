import Parameter from "./Parameter";

export const Parameters = ({
  parameters,
  onToggle,
  changeInput,
  clearInput,
  segmentNumber,
  toggleUnits,
  countUnknowns,
}) => {
  let knownsUnknowns = countUnknowns(segmentNumber);

  return (
    <>
      {parameters.map((parameter) => (
        <Parameter
          parameter={parameter}
          key={parameter.symbol}
          onToggle={onToggle}
          changeInput={changeInput}
          clearInput={clearInput}
          segmentNumber={segmentNumber}
          toggleUnits={toggleUnits}
        />
      ))}
      <div className="knownsUnknowns">
        There are {knownsUnknowns[0]} knowns and {knownsUnknowns[1]} unknowns.
      </div>
    </>
  );
};

export default Parameters;
