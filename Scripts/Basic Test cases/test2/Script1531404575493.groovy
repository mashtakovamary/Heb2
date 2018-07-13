import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.click(findTestObject('object2/span_Fruit  Vegetables (2)'))

WebUI.click(findTestObject('object2/a_Cabbage and Greens (1)'))

WebUI.click(findTestObject('object2/a_Log In (2)'))

WebUI.setText(findTestObject('object2/input_username (3)'), 'xenu@amail.club')

WebUI.setText(findTestObject('object2/input_password (3)'), 'milochka1996')

WebUI.click(findTestObject('object2/input_gigya-input-submit (2)'))

WebUI.click(findTestObject('object2/button_Change'))

WebUI.click(findTestObject('object2/a_Save for Later Lists (1)'))

WebUI.click(findTestObject('object2/span_Create New List'))

WebUI.click(findTestObject('object2/div_Home'))

WebUI.setText(findTestObject('object2/input_name (1)'), 'testtest')

WebUI.click(findTestObject('object2/button_Create List'))

WebUI.click(findTestObject('object2/a_Details'))

WebUI.click(findTestObject('object2/div_There are no products in y'))

WebUI.click(findTestObject('object2/div_There are no products in y'))

WebUI.closeBrowser()

