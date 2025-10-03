interface JSON {
  /**
   * Converts a JavaScript Object Notation (JSON) string into an object.
   * @param text A valid JSON string.
   * @param reviver A function that transforms the results. This function is called for each member of the object.
   * If a member contains nested objects, the nested objects are transformed before the parent object is.
   * @param context A context object that holds state relevant to the current expression being revived.
   * @throws {SyntaxError} If `text` is not valid JSON.
   */
  parse(
    text: string,
    reviver?: (
      this: unknown,
      key: string,
      value: unknown,
      context: ReviverContext
    ) => unknown
  ): unknown;
}

interface ReviverContext {
  source: string;
}
