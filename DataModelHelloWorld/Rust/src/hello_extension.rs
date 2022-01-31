#![allow(non_snake_case)]

// Workaround for implement proc macro
use windows as Windows;

use windows::{
    core::{implement, Result},
    Win32::{
        Foundation::{BSTR, E_NOTIMPL, PWSTR},
        System::Diagnostics::Debug::{
            IDataModelConcept, IDataModelManager, IDebugHost, IDebugHostSymbolEnumerator,
            IDebugHostTypeSignature, IKeyStore, IModelObject,
        },
    },
};

// HelloData:
//
// The context data which backs the object returned from the 'Hello' property.
//
// [JavaScript: This is partially equivalent to the __HelloObject class]
// [C++17     : This is partially equivalent to the HelloObject and Details::Hello classes]
//
#[implement()] // IUnknown is implicit
struct HelloData(String);

impl HelloData {
    fn new(text: impl ToString) -> Self {
        HelloData(text.to_string())
    }

    fn text(&self) -> &str {
        &self.0
    }
}

// HelloProperty:
//
// A property accessor for the 'Hello' property that is added to the notion of a process.
//
// [JavaScript: This is equivalent to the 'get Hello()' property of the __HelloExtension class]
// [C++17     : This is equivalent to the Get_Hello method on the HelloExtension class]
//
#[implement(Windows::Win32::System::Diagnostics::Debug::IModelPropertyAccessor)]
struct HelloProperty {
    // The data model we create to represent the type we return.  This could be stored more globally
    // than the property accessor if it's used elsewhere.
    hello_type: IModelObject,
}

impl HelloProperty {
    fn new(manager: IDataModelManager) -> Result<HelloProperty> {
        let hello_model = HelloModel;
        let hello_type =
            unsafe { manager.CreateDataModelObject(hello_model.cast::<IDataModelConcept>()?)? };

        //
        // Create a new property 'World' and place it on the model
        //
        let world_property = WorldProperty::new()

        /*
	ComPtr<WorldProperty> spWorldProperty;
	IfFailedReturn(MakeAndInitialize<WorldProperty>(&spWorldProperty, m_spHelloType.Get()));

	ComPtr<IModelObject> spWorldPropertyObject;
	IfFailedReturn(CreateProperty(spWorldProperty.Get(), &spWorldPropertyObject));
	IfFailedReturn(m_spHelloType->SetKey(L"World", spWorldPropertyObject.Get(), nullptr));

	//
	// Create a new property 'Test' and place it on the model.
	//
	ComPtr<TestProperty> spTestProperty;
	IfFailedReturn(MakeAndInitialize<TestProperty>(&spTestProperty));

	ComPtr<IModelObject> spTestPropertyObject;
	IfFailedReturn(CreateProperty(spTestProperty.Get(), &spTestPropertyObject));
	IfFailedReturn(m_spHelloType->SetKey(L"Test", spTestPropertyObject.Get(), nullptr));

	//
	// Create a string conversion and place it on the model.
	//
	ComPtr<HelloStringConversion> spHelloStringConversion;
	IfFailedReturn(MakeAndInitialize<HelloStringConversion>(&spHelloStringConversion, m_spHelloType.Get()));
	IfFailedReturn(m_spHelloType->SetConcept(__uuidof(IStringDisplayableConcept),
		static_cast<IStringDisplayableConcept *>(spHelloStringConversion.Get()),
		nullptr));
         */
        Ok(HelloProperty { hello_type })
    }

    fn GetValue(&self, _key: PWSTR, _contextobject: &Option<IModelObject>) -> Result<IModelObject> {
        Err(E_NOTIMPL.into())
    }

    // SetValue():
    //
    // Sets the value of the 'Hello' property.  This is a read only property and hence
    // the method returns E_NOTIMPL.
    //
    fn SetValue(
        &mut self,
        _key: PWSTR,
        _contextobject: &Option<IModelObject>,
        _value: &Option<IModelObject>,
    ) -> Result<()> {
        Err(E_NOTIMPL.into())
    }
}

// TestProperty:
//
// A property accessor for the 'Test' property that is added to our 'World' type.
//
// [JavaScript: This is equivalent to the 'get Test()' property of the __HelloObject class]
// [C++17     : This is equivalent to the Get_Test method on the HelloObject class]
#[implement(Windows::Win32::System::Diagnostics::Debug::IModelPropertyAccessor)]
struct TestProperty;

impl TestProperty {
    fn GetValue(&self, _key: PWSTR, _context: &Option<IModelObject>) -> Result<IModelObject> {
        //TODO
        Err(E_NOTIMPL.into())
    }

    // SetValue():
    //
    // Sets the value of the 'Test' property.  This is a read only property and hence
    // the method returns E_NOTIMPL.
    //
    fn SetValue(
        &mut self,
        _key: PWSTR,
        _context: &Option<IModelObject>,
        _value: &Option<IModelObject>,
    ) -> Result<()> {
        Err(E_NOTIMPL.into())
    }
}

// WorldProperty:
//
// A property accessor for the 'World' property that is added to our 'World' type.
//
// [JavaScript: This is equivalent to the 'get World()' property of the __HelloObject class]
// [C++17     : This is implemented via data binding in the HelloObject constructor]
//
#[implement(Windows::Win32::System::Diagnostics::Debug::IModelPropertyAccessor)]
struct WorldProperty {
    data_model: IModelObject,
}

impl WorldProperty {
    fn new(data_model: IModelObject) -> WorldProperty {
        WorldProperty { data_model }
    }

    fn GetValue(&self, _key: PWSTR, _context: &Option<IModelObject>) -> Result<IModelObject> {
        //TODO
        Err(E_NOTIMPL.into())
    }

    // SetValue():
    //
    // Sets the value of the 'World' property.  This is a read only property and hence
    // the method returns E_NOTIMPL.
    fn SetValue(
        &mut self,
        _key: PWSTR,
        _context: &Option<IModelObject>,
        _value: &Option<IModelObject>,
    ) -> Result<()> {
        Err(E_NOTIMPL.into())
    }
}

// HelloModel:
//
// The IDataModelConcept implementation for the data model which acts much like a "type" for the object
// we are returning from the 'Hello' property.
//
// [JavaScript: This is partially equivalent to the __HelloObject class]
// [C++17     : This is partially equivalent to the HelloObject class]
//
#[implement(Windows::Win32::System::Diagnostics::Debug::IDataModelConcept)]
struct HelloModel;

impl HelloModel {
    // InitializeObject():
    //
    // If the model is attached to a native object through a type signature, this method will be called
    // on each data model to indicate which type signature matched and what concrete symbols matched
    // wildcards within the type signature.  This method provides an opportunity for implementations to
    // cache attributes of the match.
    //
    fn InitializeObject(
        &self,
        _modelobject: &Option<IModelObject>,
        _matchingtypesignature: &Option<IDebugHostTypeSignature>,
        _wildcardmatches: &Option<IDebugHostSymbolEnumerator>,
    ) -> Result<()> {
        Ok(())
    }

    // GetName():
    //
    // If this model exposes itself as an extensibility point (similar to "Debugger.Models.Process"), this
    // must return the name the model is registered under; otherwise, the method returns E_NOTIMPL.
    //
    fn GetName(&self) -> Result<BSTR> {
        Err(E_NOTIMPL.into())
    }
}

// HelloExtensionModel:
//
// The IDataModelConcept implementation for our data model which extends process.  Every object which is
// attached to another in the parent model hierarchy must implement the IDataModelConcept concept.
//
// [JavaScript: This is partially equivalent to the __HelloExtension class]
// [C++17: This is partially equivalent to the HelloExtension class]
//
#[implement(Windows::Win32::System::Diagnostics::Debug::IDataModelConcept)]
struct HelloExtensionModel;

impl HelloExtensionModel {
    // InitializeObject():
    //
    // If the model is attached to a native object through a type signature, this method will be called
    // on each data model to indicate which type signature matched and what concrete symbols matched
    // wildcards within the type signature.  This method provides an opportunity for implementations to
    // cache attributes of the match.
    //
    fn InitializeObject(
        &self,
        _modelobject: &Option<IModelObject>,
        _matchingtypesignature: &Option<IDebugHostTypeSignature>,
        _wildcardmatches: &Option<IDebugHostSymbolEnumerator>,
    ) -> Result<()> {
        Ok(())
    }

    // GetName():
    //
    // If this model exposes itself as an extensibility point (similar to "Debugger.Models.Process"), this
    // must return the name the model is registered under; otherwise, the method returns E_NOTIMPL.
    //
    fn GetName(&self) -> Result<BSTR> {
        Err(E_NOTIMPL.into())
    }
}

// HelloStringConversion:
//
// The IStringDisplayableConcept implementation for the Hello model which allows an instance of that model
// to be converted to a display string for the debugger.
//
// [JavaScript: This is equivalent to the toString method on the __HelloObject class]
// [C++17     : This is equivalent to the GetStringConversion method on the HelloObject class]
//
#[implement(Windows::Win32::System::Diagnostics::Debug::IStringDisplayableConcept)]
struct HelloStringConversion {
    // Weak back pointer to the data model that contains this concept
    data_model: IModelObject,
}

impl HelloStringConversion {
    fn new(data_model: IModelObject) -> HelloStringConversion {
        HelloStringConversion { data_model }
    }

    // ToDisplayString():
    //
    // Converts an instance of the "Hello" object to a display string.
    //
    fn ToDisplayString(
        &self,
        _instance: &Option<IModelObject>,
        _metadata: &Option<IKeyStore>,
    ) -> Result<BSTR> {
        Err(E_NOTIMPL.into())
    }
}

// HelloExtension:
//
// A collection of the extensibility points that this extension places on the debugger's notion of a process.
//
// [JavaScript: This is represented by the script itself and the initializeScript method]
// [C++17     : This is equivalent to the HelloProvider and ExtensionProvider classes]
//
pub struct HelloExtension {
    manager: IDataModelManager,
    host: IDebugHost,
    process_model: IModelObject,
    hello_extension_model_object: IModelObject,
}

impl HelloExtension {
    pub fn new(manager: IDataModelManager, host: IDebugHost) -> Result<Self> {
        //
        // Get access to what is registered under "Debugger.Models.Process" and extend it.
        //
        let process_model = unsafe { manager.AcquireNamedModel("Debugger.Models.Process")? };

        //
        // Create a new object which will be added as a parent model to "Debugger.Models.Process".  This new object
        // will have all our extensibility points placed on it.  The singular link between the process model and this
        // object makes it very easy to both add our extension and remove our extension.
        //
        // Any object which is added in the parent model hierarchy must have a data model concept added.
        //
        let hello_extension_model = HelloExtensionModel;

        let hello_extension_model_object = unsafe {
            manager.CreateDataModelObject(hello_extension_model.cast::<IDataModelConcept>()?)?
        };

        //
        // Create a new property 'Hello' and place it on the extension model.
        //
        let hello_property = HelloProperty::new(manager.clone())?;

        /*

        ComPtr<IModelObject> spHelloPropertyObject;
        IfFailedReturn(CreateProperty(spHelloProperty.Get(), &spHelloPropertyObject));
        IfFailedReturn(spHelloExtensionModelObject->SetKey(L"Hello", spHelloPropertyObject.Get(), nullptr));

        IfFailedReturn(m_spProcessModelObject->AddParentModel(spHelloExtensionModelObject.Get(), nullptr, false));
        m_spHelloExtensionModelObject = std::move(spHelloExtensionModelObject);

             */

        Ok(HelloExtension {
            manager,
            host,
            process_model,
            hello_extension_model_object,
        })
    }

    pub fn initialize(&mut self) -> Result<()> {
        Ok(())
    }
}

/*
class HelloExtension
{
public:

~HelloExtension()
{
Uninitialize();
}

{HRESULT Initialize();
void Uninitialize();

private:

Microsoft::WRL::ComPtr<IModelObject> m_spProcessModelObject;
Microsoft::WRL::ComPtr<IModelObject> m_spHelloExtensionModelObject;

};

 */
