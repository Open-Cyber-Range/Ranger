import type React from 'react';
import type {ChangeEvent} from 'react';
import {type Participant} from 'src/models/exercise';

type EntitySelectProps = {
  participants: Participant[] | undefined;
  selectedEntityKey: string | undefined;
  onChange: (selectedKey: string | undefined) => void;
};

const EntitySelect: React.FC<EntitySelectProps> = ({
  participants,
  selectedEntityKey,
  onChange,
}) => {
  if (participants === undefined) {
    return null;
  }

  const handleChange = (event: ChangeEvent<HTMLSelectElement>) => {
    const selectedKey = event.target.value;
    onChange(selectedKey);
  };

  return (
    <div className='
    bp4-html-select
    bp4-minimal
    bp4-large'
    >
      <select value={selectedEntityKey ?? ''} onChange={handleChange}>
        <option value=''>Select an entity</option>
        {participants.map(participant => (
          <option key={participant.id} value={participant.selector}>
            {participant.selector}
          </option>
        ))}
      </select>
      <span className='bp4-icon bp4-icon-double-caret-vertical'/>
    </div>
  );
};

export default EntitySelect;
