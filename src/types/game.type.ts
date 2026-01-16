export interface Player {
  nickname?: string;
  level?: number;
  gender?: string;
  class?: string;
  runes?: number;
  lastGrace?: string;
  greatRunes?: GreatRune;
  baseAttributes?: BaseAttribute;
  attributes?: Attribute;
  resistances?: Resistance;
  coordinates?: Coordinate;
  statistics?: Statistics;
}

export interface GreatRune {
  name?: string;
  activated?: boolean;
}

export interface BaseAttribute {
  hp?: number;
  maxHp?: number;
  fp?: number;
  maxFp?: number;
  stamina?: number;
  maxStamina?: number;
}

export interface Attribute {
  vigor?: number;
  mind?: number;
  endurance?: number;
  strength?: number;
  dexterity?: number;
  intelligence?: number;
  faith?: number;
  arcane?: number;
}

export interface Resistance {
  // Immunity?: Poison
  immunityPoison?: number;
  maxImmunityPoison?: number;

  // Immunity?: Scarlet Rot
  immunityScarletRot?: number;
  maxImmunityScarletRot?: number;

  // Robustness?: Hemorrhage
  robustnessHemorrhage?: number;
  maxRobustnessHemorrhage?: number;

  // Robustness?: Frostbite
  robustnessFrostbite?: number;
  maxRobustnessFrostbite?: number;

  // Vitality?: Deathblight
  vitalityDeathblight?: number;
  maxVitalityDeathblight?: number;

  // Focus?: Sleep
  focusSleep?: number;
  maxFocusSleep?: number;

  // Focus?: Madness
  focusMadness?: number;
  maxFocusMadness?: number;
}

export interface Coordinate {
  x?: number;
  y?: number;
  z?: number;
}

export interface Statistics {
  totalDeaths?: number;
  totalPlaytime?: number;
}
