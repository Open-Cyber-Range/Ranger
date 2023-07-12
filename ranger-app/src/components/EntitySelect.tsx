import type React from 'react';
import type {ChangeEvent} from 'react';
import {type Entity} from 'src/models/scenario';

type EntitySelectProps = {
  entities: Record<string, Entity> | undefined;
  selectedEntityKey: string | undefined;
  onChange: (selectedKey: string | undefined) => void;
};

const EntitySelect: React.FC<EntitySelectProps> = ({
  entities,
  selectedEntityKey,
  onChange,
}) => {
  if (entities === undefined) {
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
        {Object.keys(entities).map(key => (
          <option key={key} value={key}>
            {entities[key].name}
          </option>
        ))}
      </select>
      <span className='bp4-icon bp4-icon-double-caret-vertical'/>
    </div>
  );
};

export default EntitySelect;
